use serverlib::plugin::plugin_details::PluginDetails;

#[no_mangle]
extern fn test_function() {
	println!("[Example PLugin] Testing!");
}

#[no_mangle]
extern fn details() -> PluginDetails {
	PluginDetails {
		display_name: "ExamplePlugin",
		id: "com.example.michael_bailey",
		version: "1.0.0",
		contacts: vec![
			"Mickyb18a@gmail.com"
		]
	}
}
