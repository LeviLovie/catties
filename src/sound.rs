use fmod::Error;
use tracing::debug;

pub struct Sound {
    system: fmod::studio::System,
}

impl Sound {
    pub fn new() -> Result<Self, Error> {
        let mut builder =
            unsafe { fmod::studio::SystemBuilder::new().expect("Failed to make SystemBuilder") };
        builder
            .core_builder()
            .software_format(0, fmod::SpeakerMode::Stereo, 0)?;

        let system = builder.build(
            1024,
            fmod::studio::InitFlags::NORMAL,
            fmod::InitFlags::NORMAL,
        )?;
        debug!("System created");

        Ok(Sound { system })
    }

    pub fn load(&mut self, banks: Vec<&str>) -> Result<(), Error> {
        for bank in banks {
            let exec_path = std::env::current_exe().expect("Failed to get current executable path");
            let bank_path = exec_path
                .parent()
                .expect("Failed to get parent directory")
                .join("banks")
                .join(format!("{}.bank", bank));
            let bank_path = bank_path
                .to_str()
                .expect("Failed to convert OsStr to &str")
                .to_string();
            let bank_path = std::ffi::CString::new(bank_path).unwrap();
            let bank_path: &fmod::Utf8CStr = fmod::Utf8CStr::from_cstr(&bank_path)
                .expect("Failed to convert CString to Utf8CStr");
            self.system
                .load_bank_file(bank_path, fmod::studio::LoadBankFlags::NORMAL)?;
            debug!("Bank {} loaded", bank);
        }

        Ok(())
    }

    pub fn start(&mut self, event: &str) -> Result<(), Error> {
        let event_path = format!("event:/{}", event);
        let event_path = std::ffi::CString::new(event_path).unwrap();
        let event_path: &fmod::Utf8CStr =
            fmod::Utf8CStr::from_cstr(&event_path).expect("Failed to convert CString to Utf8CStr");
        let event_desc = match self.system.get_event(event_path) {
            Ok(event_desc) => event_desc,
            Err(e) => {
                debug!("Failed to get event description: {}", e);
                return Err(e);
            }
        };
        let event_instance = event_desc.create_instance()?;
        event_instance.start()?;
        debug!("Event {} started", event);

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Error> {
        self.system.update()?;
        Ok(())
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe { self.system.release().expect("Failed to release System") };
    }
}
