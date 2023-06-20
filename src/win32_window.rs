use std::mem::size_of;
use std::ptr;

use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM, BOOL, RECT},
        System::{
            LibraryLoader::{GetModuleHandleA, LoadLibraryA, GetProcAddress}, 
            Memory::{MEM_COMMIT, PAGE_READWRITE, MEM_RELEASE}
        },
        System::Memory::{VirtualAlloc, VirtualFree},
        UI::WindowsAndMessaging::{ 
            HMENU, WNDCLASSA, WINDOW_EX_STYLE, MSG,
            CreateWindowExA, DefWindowProcA, RegisterClassA, PeekMessageA,
            TranslateMessage, DispatchMessageA, GetClientRect,
            WM_CLOSE, WM_SIZE, WM_DESTROY, WM_ACTIVATEAPP, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
            CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, WM_PAINT, PM_REMOVE, WM_QUIT, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
        }, 
        Graphics::Gdi::{ 
            PAINTSTRUCT, HDC, BITMAPINFO, BITMAPINFOHEADER,
            BeginPaint, EndPaint, StretchDIBits, ReleaseDC, GetDC,
            DIB_RGB_COLORS, BI_RGB, SRCCOPY, 
        }
    },
    s
};

use winapi::um::{xinput::{
    XUSER_MAX_COUNT, XINPUT_GAMEPAD_DPAD_UP, XINPUT_GAMEPAD_DPAD_DOWN, XINPUT_GAMEPAD_DPAD_LEFT, 
    XINPUT_GAMEPAD_DPAD_RIGHT, XINPUT_GAMEPAD_START, XINPUT_GAMEPAD_BACK,
    XINPUT_STATE, XINPUT_VIBRATION, XInputSetState, XInputGetState
}, dsound::{DSSCL_PRIORITY, DSBCAPS_PRIMARYBUFFER}};
// Keyboard constants
use winapi::um::winuser::VK_SPACE;

use winapi::um::dsound::{DSBUFFERDESC, IDirectSoundBuffer, LPDIRECTSOUND, IDirectSound};
use winapi::shared::mmreg::{WAVEFORMATEX, WAVE_FORMAT_PCM};
use winapi::shared::minwindef::DWORD;
use winapi::shared::guiddef::{GUID, IID_NULL};
use winapi::ctypes::c_void;
use winapi::shared::winerror::SUCCEEDED;

struct OffscreenBuffer {
    info: Option<BITMAPINFO>,
    memory: Option<*mut core::ffi::c_void>,
    width: i32,
    height: i32,
    bytes_per_pixel: i32,
    pitch: usize
}

struct WindowDimension {
    width: i32,
    height: i32
}

static mut IS_RUNNING: bool = true;
static mut BACKBUFFER: OffscreenBuffer = OffscreenBuffer {
    info: None,
    memory: None,
    width: 0,
    height: 0,
    bytes_per_pixel: 4,
    pitch: 0
};

type XInputGetStateDef = unsafe extern "system" fn(u32, *mut XINPUT_STATE) -> DWORD;
type XInputSetStateDef = unsafe extern "system" fn(DWORD, *mut XINPUT_VIBRATION) -> DWORD;
type DirectSoundCreateDef = unsafe extern "system" fn(
    *const GUID,
    *mut LPDIRECTSOUND,
    *mut c_void
) -> DWORD;

static mut XINPUT_GET_STATE: Option<XInputGetStateDef> = Some(XInputGetState);
static mut XINPUT_SET_STATE: Option<XInputSetStateDef> = Some(XInputSetState);

// Maybe remove this, seems like it's unnecessary to support old versions
// most people will likely be on newer windows version so a fallback is redundant
fn load_xinput() {
    let dll_paths = vec![s!("xinput1_4.dll"), s!("xinput1_3.dll")];
    
    for dll_path in dll_paths {
        let hmodule = unsafe { LoadLibraryA(dll_path) };
        match hmodule {
            Ok(value) => {
                unsafe { 
                    XINPUT_GET_STATE = Some(std::mem::transmute(GetProcAddress(value, s!("XInputGetState"))));
                    XINPUT_SET_STATE = Some(std::mem::transmute(GetProcAddress(value, s!("XInputSetState"))));
                }
                break;
            },
            _ => continue
        };
    }
}

pub fn open_win32_window() {
    load_xinput();

    unsafe {
        resize_dib_section(&mut BACKBUFFER, 1280, 720);
    }

    let instance = unsafe { GetModuleHandleA(None).ok().unwrap() };
    let class = WNDCLASSA {
        hInstance: instance,
        lpszClassName: s!("Window Title"),
        lpfnWndProc: Some(main_window_callback),
        style: CS_HREDRAW|CS_VREDRAW,
        ..Default::default()
    };

    unsafe { 
        if RegisterClassA(&class) > 0 {
            let window_handle = 
                CreateWindowExA(            
                    WINDOW_EX_STYLE::default(), 
                    class.lpszClassName, 
                    s!("Nova Engine"), 
                    WS_OVERLAPPEDWINDOW|WS_VISIBLE,
                    CW_USEDEFAULT, 
                    CW_USEDEFAULT, 
                    CW_USEDEFAULT, 
                    CW_USEDEFAULT, 
                    HWND::default(), 
                    HMENU::default(), 
                    instance, 
                    None);

            let mut x_offset = 0;   
            let mut y_offset = 0;

            let samples_per_second: i32 = 48000;
            let buffer_size: i32 = samples_per_second * 2 * std::mem::size_of::<i16>() as i32;
            
            win32_init_direct_sound(window_handle, 48000, buffer_size);
                    
            while IS_RUNNING {
                let mut message: MSG = MSG::default();
                while PeekMessageA(&mut message, HWND::default(), 0, 0, PM_REMOVE) != BOOL::default() {
                    if message.message == WM_QUIT {
                        IS_RUNNING = false;
                    }

                    TranslateMessage(&message);
                    DispatchMessageA(&message);    
                }

                if XINPUT_GET_STATE.is_some() && XINPUT_SET_STATE.is_some()  {
                    for controller_index in 0..XUSER_MAX_COUNT {
                        let mut controller_state: XINPUT_STATE = std::mem::zeroed();
                        if XINPUT_GET_STATE.unwrap()(controller_index, &mut controller_state) == 0 {
                            // Retrieved controller state
                            // Check if dwPacketNumber increases too rapidly
                            let dpad_up = controller_state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_UP;
                            let dpad_down = controller_state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_DOWN;
                            let dpad_left = controller_state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_LEFT;
                            let dpad_right = controller_state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_RIGHT;
                            let dpad_start = controller_state.Gamepad.wButtons & XINPUT_GAMEPAD_START;
                            let dpad_back = controller_state.Gamepad.wButtons & XINPUT_GAMEPAD_BACK;

                            let stick_x = controller_state.Gamepad.sThumbLX;
                            let stick_y = controller_state.Gamepad.sThumbLY;

                            if dpad_up == 1 {
                                y_offset += 2;
                            }
                        } else {
                            // Controller not available
                        }

                    }
                }

                // let mut vibration = XINPUT_VIBRATION {
                //     wLeftMotorSpeed: 60000,
                //     wRightMotorSpeed: 60000
                // };
                // unsafe {
                //     XINPUT_SET_STATE.unwrap()(0, &mut vibration);
                // }

                render(&mut BACKBUFFER, x_offset, y_offset);

                let device_context = GetDC(window_handle);

                let window_dimension = get_window_dimension(window_handle);

                display_buffer_in_window(device_context, window_dimension.width, window_dimension.height,
                     &BACKBUFFER, 0, 0, window_dimension.width, window_dimension.height);
                ReleaseDC(window_handle, device_context);
                
                x_offset += 1;
                y_offset += 1;
            } 
        } else {
            //todo!("logging");
        }

        //todo!("logging");
    }
}

unsafe extern "system" fn main_window_callback(
    window: HWND,
    message: u32, 
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {    
    let mut result = LRESULT::default();

    match message {
        WM_SIZE => { 
            println!("WM_SIZE");
        },
        WM_CLOSE => { 
            // DestroyWindow(window);
            IS_RUNNING = false;
            println!("WM_CLOSE"); 
        },
        WM_DESTROY => { 
            // PostQuitMessage(0);
            IS_RUNNING = false;
            println!("WM_DESTROY");
        },
        WM_ACTIVATEAPP => { 
            println!("WM_ACTIVATEAPP");
        },
        WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => {
            // SYSKEYUP/DOWN is for function/alt/ctrl
            // need to manually handle alt+f4

            let virtual_key_code = wparam.0 as u32;
            let was_key_pressed = (lparam.0 as u32) & (1 << 30) != 0;
            let is_key_down = (lparam.0 as u32) & (1 << 31) == 0;

            if was_key_pressed != is_key_down {
                if virtual_key_code == 'W' as u32 {
                    println!("W was pressed");
                } 
                if virtual_key_code == VK_SPACE as u32 {
                    println!("Space was pressed");
                }
                println!("Was it pressed: {}", was_key_pressed);
                println!("Is down?: {}", is_key_down);    
            }
        },
        WM_PAINT => {
            let mut paint_struct = PAINTSTRUCT::default();
            let device_context = BeginPaint(window, &mut paint_struct);

            let x = paint_struct.rcPaint.left; 
            let y = paint_struct.rcPaint.top;
            let width = paint_struct.rcPaint.right - paint_struct.rcPaint.left;
            let height = paint_struct.rcPaint.bottom - paint_struct.rcPaint.top;

            let window_dimension = get_window_dimension(window);

            display_buffer_in_window(device_context, window_dimension.width, window_dimension.height, 
                &BACKBUFFER, x, y, width, height);

            EndPaint(window, &paint_struct);
        },
        _ => { 
            result = DefWindowProcA(window, message, wparam, lparam);
        }
    }

    result
}

fn render(offscreen_buffer: &OffscreenBuffer, x_offset: i32, y_offset: i32) {
    unsafe {
        
        match offscreen_buffer.memory {
            Some(value) => {               
                let mut row = value as *mut u8;

                for y in 0..offscreen_buffer.height {
                    let mut pixel = row as *mut u32;

                    for x in 0..offscreen_buffer.width {
                        let blue: u8 = (x + x_offset) as u8;
                        let green: u8 = (y + y_offset) as u8;
                        let red: u8 = 0;
                        
                        *pixel = (u32::from(red) << 16) | (u32::from(green) << 8) | u32::from(blue);
                        pixel = pixel.add(1);
                    }

                    row = row.add(offscreen_buffer.pitch);
                }
            },
            None => return
        }
    }
}

fn resize_dib_section(offscreen_buffer: &mut OffscreenBuffer, width: i32, height: i32) {
    unsafe {
        if offscreen_buffer.memory.is_some() {
            VirtualFree(offscreen_buffer.memory.unwrap(), 0, MEM_RELEASE);
        }

        let mut bitmap_info: BITMAPINFO = BITMAPINFO::default();

        offscreen_buffer.width = width;
        offscreen_buffer.height = height;
        offscreen_buffer.bytes_per_pixel = 4;
    
        let mut bitmap_header_info = BITMAPINFOHEADER::default();
        bitmap_header_info.biSize = size_of::<BITMAPINFOHEADER>() as u32;
        bitmap_header_info.biWidth = offscreen_buffer.width;
        bitmap_header_info.biHeight = -offscreen_buffer.height;
        bitmap_header_info.biPlanes = 1;
        bitmap_header_info.biBitCount = 32;
        bitmap_header_info.biCompression =  BI_RGB.0 as u32; // 0 or uncompressed
        bitmap_info.bmiHeader = bitmap_header_info;

        offscreen_buffer.info = Some(bitmap_info);
    }

    let bitmap_memory_size: usize = 
        (offscreen_buffer.width * offscreen_buffer.height * offscreen_buffer.bytes_per_pixel).try_into().unwrap();

    unsafe {
        offscreen_buffer.memory = 
            Some(VirtualAlloc(None, bitmap_memory_size, MEM_COMMIT, PAGE_READWRITE));
    }

    offscreen_buffer.pitch = (offscreen_buffer.width * offscreen_buffer.bytes_per_pixel) as usize;
} 

fn display_buffer_in_window(
    device_context: HDC, 
    window_width: i32, 
    window_height: i32, 
    offscreen_buffer: &OffscreenBuffer, 
    x: i32,
    y: i32, 
    width: i32, 
    height: i32) {
    unsafe {
        let bit_memory = offscreen_buffer.memory.map(|p| p as *const core::ffi::c_void);
        StretchDIBits(
            device_context, 
            0, 0, window_width, window_height,
            0, 0, offscreen_buffer.width, offscreen_buffer.height,
            bit_memory, &offscreen_buffer.info.unwrap(), DIB_RGB_COLORS, SRCCOPY);
    }
}

fn get_window_dimension(window: HWND) -> WindowDimension {
    let mut client_rect: RECT = RECT::default();
    unsafe {
        GetClientRect(window, &mut client_rect);
    }
    
    let width = client_rect.right - client_rect.left;
    let height = client_rect.bottom - client_rect.top;

    return WindowDimension {
        width,
        height
    }
}

fn win32_init_direct_sound(window: HWND, samples_per_second: i32, buffer_size: i32) {
    let hmodule = unsafe { LoadLibraryA(s!("dsound.dll")) };

    match hmodule {
        Ok(value) => {
            unsafe { 
                let direct_sound_create: DirectSoundCreateDef 
                    = std::mem::transmute(GetProcAddress(value, s!("DirectSoundCreate")));

                let mut direct_sound: *mut IDirectSound = ptr::null_mut();

                if SUCCEEDED(direct_sound_create(ptr::null_mut(), &mut direct_sound, ptr::null_mut()) as winapi::shared::winerror::HRESULT) {
                    if SUCCEEDED((*direct_sound).SetCooperativeLevel(window.0 as winapi::shared::windef::HWND, DSSCL_PRIORITY)) {
                        // Specify the wave format
                        let mut wave_format: WAVEFORMATEX = WAVEFORMATEX {
                            wFormatTag: WAVE_FORMAT_PCM,
                            nChannels: 2,
                            nSamplesPerSec: samples_per_second as u32,
                            wBitsPerSample: 16,
                            nBlockAlign: 2 * 2,
                            nAvgBytesPerSec: samples_per_second as u32 * 2 * 2,
                            cbSize: 0,
                        };

                        let primary_buffer_desc: DSBUFFERDESC = DSBUFFERDESC {
                            dwSize: std::mem::size_of::<DSBUFFERDESC>() as u32,
                            dwFlags: DSBCAPS_PRIMARYBUFFER,
                            dwBufferBytes: 0, 
                            dwReserved: 0,
                            lpwfxFormat: ptr::null_mut(),
                            guid3DAlgorithm: IID_NULL
                        };

                        // Create primary buffer
                        let mut primary_buffer: *mut IDirectSoundBuffer = ptr::null_mut();

                        if SUCCEEDED((*direct_sound).CreateSoundBuffer(&primary_buffer_desc, &mut primary_buffer, ptr::null_mut())) {
                            if SUCCEEDED((*primary_buffer).SetFormat(&mut wave_format)) {
                                println!("Format set");
                            } else {

                            }
                        } else {
                            
                        }

                        let secondary_buffer_desc: DSBUFFERDESC = DSBUFFERDESC {
                            dwSize: std::mem::size_of::<DSBUFFERDESC>() as DWORD,
                            dwFlags: 0,
                            dwBufferBytes: buffer_size as u32, 
                            dwReserved: 0,
                            lpwfxFormat: &mut wave_format,
                            guid3DAlgorithm: IID_NULL
                        };
                        
                        // Create secondary buffer
                        let mut secondary_buffer: *mut IDirectSoundBuffer = ptr::null_mut();

                        if SUCCEEDED((*direct_sound).CreateSoundBuffer(&secondary_buffer_desc, &mut secondary_buffer, ptr::null_mut())) {
                            println!("Secondary Buffer Set");
                        } else {
                            
                        }
                    }   
                } else {

                }
            }
        }
        _ => return
    };
}