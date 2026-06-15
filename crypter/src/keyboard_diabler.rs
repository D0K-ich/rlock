use std::mem::size_of;
use windows::core::{GUID, PCWSTR};
use windows::Win32::Devices::DeviceAndDriverInstallation::{
	CM_Disable_DevNode, CM_Enable_DevNode, CM_Locate_DevNodeW, SetupDiDestroyDeviceInfoList,
	SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, SetupDiGetDeviceInstanceIdW,
	SetupDiGetDeviceRegistryPropertyW, CM_LOCATE_DEVNODE_FLAGS,
	CR_SUCCESS, DIGCF_ALLCLASSES, DIGCF_PRESENT, HDEVINFO, SPDRP_CLASS,
	SPDRP_DEVICEDESC, SPDRP_FRIENDLYNAME, SPDRP_HARDWAREID, SP_DEVINFO_DATA,
};

#[derive(Debug, Clone)]
pub struct KeyboardDevice {
	pub instance_id: String,
	pub description: String,
	pub dev_node: u32,
}

pub struct KeyboardController;

impl KeyboardController {
	pub fn find_keyboards() -> Result<Vec<KeyboardDevice>, Box<dyn std::error::Error>> {
		let mut keyboards = Vec::new();

		unsafe {
			let device_info_set = SetupDiGetClassDevsW(None::<*const GUID>, PCWSTR::null(), None, DIGCF_PRESENT | DIGCF_ALLCLASSES, )?;

			let mut device_info_data = SP_DEVINFO_DATA::default();
			device_info_data.cbSize = size_of::<SP_DEVINFO_DATA>() as u32;
			let mut device_index = 0;

			println!("Scanning all devices...");
			println!("{:^10} | {:^20} | {:^40}", "Index", "Class", "Description");
			println!("{:-<10}-+-{:-<20}-+-{:-<40}", "", "", "");

			while SetupDiEnumDeviceInfo(device_info_set, device_index, &mut device_info_data).is_ok() {
				if let Some(class_name) = Self::get_device_class(device_info_set, &device_info_data) {
					let description = Self::get_device_description(device_info_set, &device_info_data);
					let friendly_name = Self::get_device_friendly_name(device_info_set, &device_info_data);
					let device_desc = Self::get_device_desc(device_info_set, &device_info_data);

					let combined_desc = format!("{} {} {}", description, friendly_name, device_desc);

					println!("{:^10} | {:^20} | {:.40}", device_index, class_name, combined_desc);

					let is_keyboard = class_name == "Keyboard"
						|| class_name == "KeyboardClass"
						|| class_name == "HIDClass"
						|| class_name.to_lowercase().contains("keyboard")
						|| class_name.to_lowercase().contains("kbd")
						|| combined_desc.to_lowercase().contains("keyboard")
						|| combined_desc.to_lowercase().contains("ps/2");

					if is_keyboard {
						println!(">>> FOUND KEYBOARD! Index: {}, Class: {}", device_index, class_name);

						if let Ok(instance_id) = Self::get_device_instance_id(device_info_set, &device_info_data) {
							let final_desc = if !device_desc.is_empty() {
								device_desc
							} else if !friendly_name.is_empty() {
								friendly_name
							} else if !description.is_empty() {
								description
							} else {
								format!("{} Device", class_name)
							};

							println!("    Instance ID: {}", instance_id);
							println!("    Description: {}", final_desc);

							keyboards.push(KeyboardDevice { instance_id, description: final_desc, dev_node: device_info_data.DevInst, });
						}
					}
				}
				device_index += 1;
			}

			let _ = SetupDiDestroyDeviceInfoList(device_info_set);
		}

		if keyboards.is_empty() {
			println!("\n⚠️  No keyboards found in standard scan!");
			println!("Trying alternative method...");
			return Self::find_keyboards_alternative();
		}

		Ok(keyboards)
	}

	fn find_keyboards_alternative() -> Result<Vec<KeyboardDevice>, Box<dyn std::error::Error>> {
		let mut keyboards = Vec::new();

		unsafe {
			let device_info_set = SetupDiGetClassDevsW(
				None::<*const GUID>,
				PCWSTR::null(),
				None,
				DIGCF_PRESENT | DIGCF_ALLCLASSES,
			)?;

			let mut device_info_data = SP_DEVINFO_DATA::default();
			device_info_data.cbSize = size_of::<SP_DEVINFO_DATA>() as u32;
			let mut device_index = 0;

			println!("\nAlternative scan - looking for any input devices...");

			while SetupDiEnumDeviceInfo(device_info_set, device_index, &mut device_info_data).is_ok() {
				let description = Self::get_device_desc(device_info_set, &device_info_data);
				let friendly_name = Self::get_device_friendly_name(device_info_set, &device_info_data);
				let hardware_id = Self::get_device_description(device_info_set, &device_info_data);

				let combined = format!("{} {} {}", description, friendly_name, hardware_id).to_lowercase();

				if combined.contains("keyboard")
					|| combined.contains("kbd")
					|| combined.contains("ps/2")
					|| combined.contains("101")
					|| combined.contains("102")
					|| (combined.contains("hid") && combined.contains("key")) {

					if let Ok(instance_id) = Self::get_device_instance_id(device_info_set, &device_info_data) {
						let final_desc = if !description.is_empty() {
							description
						} else if !friendly_name.is_empty() {
							friendly_name
						} else {
							"Unknown Keyboard".to_string()
						};

						println!("  Found potential keyboard: {}", final_desc);
						println!("    Instance ID: {}", instance_id);

						keyboards.push(KeyboardDevice { instance_id, description: final_desc, dev_node: device_info_data.DevInst, });
					}
				}
				device_index += 1;
			}

			let _ = SetupDiDestroyDeviceInfoList(device_info_set);
		}

		if keyboards.is_empty() {
			println!("\nTrying WMI fallback...");
			return Self::find_keyboards_wmi();
		}

		Ok(keyboards)
	}

	fn find_keyboards_wmi() -> Result<Vec<KeyboardDevice>, Box<dyn std::error::Error>> {
		let mut keyboards = Vec::new();

		let output = std::process::Command::new("powershell")
			.args(&["-Command", "Get-WmiObject Win32_Keyboard | Select-Object PNPDeviceID, Name | ConvertTo-Csv -NoTypeInformation"])
			.output()?;

		if output.status.success() {
			let csv = String::from_utf8_lossy(&output.stdout);
			for line in csv.lines().skip(1) { 
				let parts: Vec<&str> = line.split(',').collect();
				if parts.len() >= 2 {
					let instance_id = parts[0].trim_matches('"').to_string();
					let description = parts[1].trim_matches('"').to_string();

					if !instance_id.is_empty() {
						println!("  Found via WMI: {}", description);
						keyboards.push(KeyboardDevice {
							instance_id,
							description,
							dev_node: 0,
						});
					}
				}
			}
		}

		Ok(keyboards)
	}

	unsafe fn get_device_class(device_info_set: HDEVINFO, device_info_data: &SP_DEVINFO_DATA, ) -> Option<String> {
		let mut required_size = 0u32;

		let result = SetupDiGetDeviceRegistryPropertyW(
			device_info_set,
			device_info_data,
			SPDRP_CLASS,
			Some(&mut required_size),
			None,
			Some(&mut required_size),
		);

		if result.is_ok() && required_size > 0 {
			let mut buffer = vec![0u16; (required_size / 2 + 1) as usize];

			let result2 = SetupDiGetDeviceRegistryPropertyW(
				device_info_set,
				device_info_data,
				SPDRP_CLASS,
				Some(&mut required_size),
				Some(&mut std::slice::from_raw_parts_mut(
					buffer.as_mut_ptr() as *mut u8,
					buffer.len() * 2,
				)),
				Some(&mut required_size),
			);

			if result2.is_ok() {
				return Some(String::from_utf16_lossy(&buffer[..(required_size as usize / 2)]));
			}
		}
		None
	}

	unsafe fn get_device_instance_id(device_info_set: HDEVINFO, device_info_data: &SP_DEVINFO_DATA, ) -> Result<String, Box<dyn std::error::Error>> {
		let mut required_size = 0u32;

		let _ = SetupDiGetDeviceInstanceIdW(
			device_info_set,
			device_info_data,
			None,
			Some(&mut required_size),
		);

		if required_size > 0 {
			let mut buffer = vec![0u16; (required_size + 1) as usize];

			SetupDiGetDeviceInstanceIdW(
				device_info_set,
				device_info_data,
				Some(&mut buffer),
				Some(&mut required_size),
			)?;

			Ok(String::from_utf16_lossy(&buffer[..required_size as usize]))
		} else {
			Err("Failed to get instance ID".into())
		}
	}

	unsafe fn get_device_description(device_info_set: HDEVINFO, device_info_data: &SP_DEVINFO_DATA, ) -> String {
		let mut required_size = 0u32;
		let mut buffer = [0u16; 512];

		let result = SetupDiGetDeviceRegistryPropertyW(
			device_info_set,
			device_info_data,
			SPDRP_HARDWAREID,
			Some(&mut required_size),
			Some(&mut std::slice::from_raw_parts_mut(
				buffer.as_mut_ptr() as *mut u8,
				buffer.len() * 2,
			)),
			Some(&mut required_size),
		);

		if result.is_ok() && required_size > 0 { String::from_utf16_lossy(&buffer[..(required_size as usize / 2)]) } else { String::new() }
	}

	unsafe fn get_device_desc(device_info_set: HDEVINFO, device_info_data: &SP_DEVINFO_DATA, ) -> String {
		let mut required_size = 0u32;
		let mut buffer = [0u16; 512];

		let result = SetupDiGetDeviceRegistryPropertyW(
			device_info_set,
			device_info_data,
			SPDRP_DEVICEDESC,
			Some(&mut required_size),
			Some(&mut std::slice::from_raw_parts_mut(
				buffer.as_mut_ptr() as *mut u8,
				buffer.len() * 2,
			)),
			Some(&mut required_size),
		);

		if result.is_ok() && required_size > 0 { String::from_utf16_lossy(&buffer[..(required_size as usize / 2)]) } else { String::new() }
	}

	unsafe fn get_device_friendly_name(device_info_set: HDEVINFO, device_info_data: &SP_DEVINFO_DATA, ) -> String {
		let mut required_size = 0u32;
		let mut buffer = [0u16; 512];

		let result = SetupDiGetDeviceRegistryPropertyW(
			device_info_set,
			device_info_data,
			SPDRP_FRIENDLYNAME,
			Some(&mut required_size),
			Some(&mut std::slice::from_raw_parts_mut(
				buffer.as_mut_ptr() as *mut u8,
				buffer.len() * 2,
			)),
			Some(&mut required_size),
		);

		if result.is_ok() && required_size > 0 { String::from_utf16_lossy(&buffer[..(required_size as usize / 2)]) } else { String::new() }
	}

	pub fn disable_keyboard(instance_id: &str) -> Result<(), Box<dyn std::error::Error>> {
		unsafe {
			let wide_id: Vec<u16> = instance_id.encode_utf16().chain(Some(0)).collect();
			let mut dev_node = 0u32;

			let result = CM_Locate_DevNodeW(&mut dev_node, PCWSTR(wide_id.as_ptr()), CM_LOCATE_DEVNODE_FLAGS(0));
			if result != CR_SUCCESS { return Err(format!("Failed to locate device node: {:?}", result).into()); }

			let result = CM_Disable_DevNode(dev_node, 0);
			if result != CR_SUCCESS { return Err(format!("Failed to disable device: {:?}", result).into()); }

			println!("✅ Keyboard disabled successfully");
			Ok(())
		}
	}

	pub fn enable_keyboard(instance_id: &str) -> Result<(), Box<dyn std::error::Error>> {
		unsafe {
			let wide_id: Vec<u16> = instance_id.encode_utf16().chain(Some(0)).collect();
			let mut dev_node = 0u32;

			let result = CM_Locate_DevNodeW(&mut dev_node, PCWSTR(wide_id.as_ptr()), CM_LOCATE_DEVNODE_FLAGS(0));
			if result != CR_SUCCESS { return Err(format!("Failed to locate device node: {:?}", result).into()); }

			let result = CM_Enable_DevNode(dev_node, 0);
			if result != CR_SUCCESS { return Err(format!("Failed to enable device: {:?}", result).into()); }

			println!("✅ Keyboard enabled successfully");
			Ok(())
		}
	}

	pub fn disable_first_keyboard() -> Result<(), Box<dyn std::error::Error>> {
		let keyboards = Self::find_keyboards()?;
		if keyboards.is_empty() { return Err("No keyboards found".into()); }
		println!("Disabling keyboard: {}", keyboards[0].description);
		Self::disable_keyboard(&keyboards[0].instance_id)
	}

	pub fn enable_first_keyboard() -> Result<(), Box<dyn std::error::Error>> {
		let keyboards = Self::find_keyboards()?;
		if keyboards.is_empty() { return Err("No keyboards found".into()); }
		
		println!("Enabling keyboard: {}", keyboards[0].description);
		Self::enable_keyboard(&keyboards[0].instance_id)
	}
}
