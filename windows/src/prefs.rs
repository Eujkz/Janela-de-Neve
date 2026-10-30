use std::ffi::c_void;

use windows::Win32::Foundation::{ERROR_SUCCESS, WIN32_ERROR};
use windows::Win32::System::Registry::{
    HKEY_CURRENT_USER, REG_DWORD, REG_SZ, RRF_RT_REG_DWORD, RRF_RT_REG_SZ, RegDeleteKeyValueW, RegGetValueW, RegSetKeyValueW,
};
use windows::Win32::UI::WindowsAndMessaging::{SPI_GETCLIENTAREAANIMATION, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SystemParametersInfoW};
use windows::core::{BOOL, PCWSTR, w};

use snowhush_core::simulation::Snowfall;
use snowhush_core::mod_config::ModProfile; // SNOWHUSH_MODKIT

const KEY: PCWSTR = w!("Software\\Snowhush");
const RUN: PCWSTR = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
const RUN_VALUE: PCWSTR = w!("Snowhush");

#[derive(Clone, Copy)]
pub struct Settings {
    pub snowfall: Snowfall,
    pub accumulate: bool,
    pub melt: bool,
    pub snow_on_windows: bool,
    pub behind_windows: bool,
    pub show_in_screenshots: bool,
    pub welcomed: bool,
    pub mod_profile: ModProfile,
}

impl Settings {
    pub fn load() -> Settings {
        let calm = !animations_enabled();
        let snowfall = read(w!("snowfall")).and_then(|i| Snowfall::ALL.get(i as usize).copied());
        Settings {
            snowfall: snowfall.unwrap_or(if calm { Snowfall::Light } else { Snowfall::Steady }),
            accumulate: read(w!("accumulate")).is_none_or(|v| v != 0),
            melt: read(w!("meltSnow")).is_none_or(|v| v != 0),
            snow_on_windows: read(w!("snowOnWindows")).is_none_or(|v| v != 0),
            behind_windows: read(w!("behindWindows")).is_some_and(|v| v != 0),
            show_in_screenshots: read(w!("showInScreenshots")).is_some_and(|v| v != 0),
            welcomed: read(w!("welcomed")).is_some_and(|v| v != 0),
            mod_profile: ModProfile {
                season: read(w!("modSeason")).unwrap_or(0).min(3),
                storm: read(w!("modStorm")).is_some_and(|v| v != 0),
                density: read(w!("modDensity")).unwrap_or(2).min(4),
                speed: read(w!("modSpeed")).unwrap_or(1).min(3),
                wind: read(w!("modWind")).unwrap_or(1).min(3),
                size: read(w!("modSize")).unwrap_or(1).min(3),
                cap: read(w!("modCap")).unwrap_or(2).min(3),
                cursor: read(w!("modCursor")).unwrap_or(1).min(3),
                color: read(w!("modColor")).unwrap_or(0).min(4),
            },
        }
    }

    pub fn save(&self) {
        let snowfall = Snowfall::ALL.iter().position(|&s| s == self.snowfall).unwrap_or(2);
        write(w!("snowfall"), snowfall as u32);
        write(w!("accumulate"), self.accumulate as u32);
        write(w!("meltSnow"), self.melt as u32);
        write(w!("snowOnWindows"), self.snow_on_windows as u32);
        write(w!("behindWindows"), self.behind_windows as u32);
        write(w!("showInScreenshots"), self.show_in_screenshots as u32);
        write(w!("welcomed"), self.welcomed as u32);
        let p = self.mod_profile;
        write(w!("modSeason"), p.season);
        write(w!("modStorm"), p.storm as u32);
        write(w!("modDensity"), p.density);
        write(w!("modSpeed"), p.speed);
        write(w!("modWind"), p.wind);
        write(w!("modSize"), p.size);
        write(w!("modCap"), p.cap);
        write(w!("modCursor"), p.cursor);
        write(w!("modColor"), p.color);
    }
}

pub fn opens_at_login() -> bool {
    let mut buffer = [0u16; 1024];
    let mut size = (buffer.len() * 2) as u32;
    let status =
        unsafe { RegGetValueW(HKEY_CURRENT_USER, RUN, RUN_VALUE, RRF_RT_REG_SZ, None, Some(buffer.as_mut_ptr().cast::<c_void>()), Some(&mut size)) };
    if status != ERROR_SUCCESS {
        return false;
    }
    let length = buffer.iter().position(|&c| c == 0).unwrap_or(0);
    command().is_some_and(|command| command[..command.len() - 1] == buffer[..length])
}

pub fn set_opens_at_login(on: bool) -> WIN32_ERROR {
    unsafe {
        if !on {
            return RegDeleteKeyValueW(HKEY_CURRENT_USER, RUN, RUN_VALUE);
        }
        let Some(command) = command() else { return WIN32_ERROR(1) };
        RegSetKeyValueW(HKEY_CURRENT_USER, RUN, RUN_VALUE, REG_SZ.0, Some(command.as_ptr().cast()), (command.len() * 2) as u32)
    }
}

/// The quoted path of this executable, NUL-terminated.
fn command() -> Option<Vec<u16>> {
    let path = std::env::current_exe().ok()?;
    Some(format!("\"{}\"", path.display()).encode_utf16().chain([0]).collect())
}

fn read(name: PCWSTR) -> Option<u32> {
    let mut value = 0u32;
    let mut size = 4u32;
    let status =
        unsafe { RegGetValueW(HKEY_CURRENT_USER, KEY, name, RRF_RT_REG_DWORD, None, Some((&raw mut value).cast::<c_void>()), Some(&mut size)) };
    (status == ERROR_SUCCESS).then_some(value)
}

fn write(name: PCWSTR, value: u32) {
    unsafe {
        let _ = RegSetKeyValueW(HKEY_CURRENT_USER, KEY, name, REG_DWORD.0, Some((&raw const value).cast::<c_void>()), 4);
    }
}

/// The "Animation effects" setting: off means the person prefers less motion.
pub fn animations_enabled() -> bool {
    let mut enabled = BOOL(1);
    let result = unsafe {
        SystemParametersInfoW(SPI_GETCLIENTAREAANIMATION, 0, Some((&raw mut enabled).cast::<c_void>()), SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0))
    };
    result.is_err() || enabled.as_bool()
}
