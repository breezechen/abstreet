initSidebarItems({"fn":[["delete_file","Idempotent"],["download_bytes","Downloads bytes from a URL. This must be called with a tokio runtime somewhere. The caller creates an mpsc channel pair and provides the sender. Progress will be described through it."],["download_to_file","Download a file from a URL. This must be called with a tokio runtime somewhere. Progress will be printed to STDOUT."],["file_exists",""],["find_next_file",""],["find_prev_file","Keeps file extensions"],["http_get","Performs an HTTP GET request and returns the raw response. Unlike the variations in download.rs, no progress – but it works on native and web."],["http_post","Performs an HTTP POST request and returns the response."],["list_all_objects","Just list all things from a directory, return sorted by name, with file extension removed."],["list_dir","Returns full paths"],["load_all_objects","Load all serialized things from a directory, return sorted by name, with file extension removed. Detects JSON or binary. Filters out broken files."],["maybe_read_binary",""],["maybe_read_json",""],["must_read_object","May be a JSON or binary file. Panics on failure."],["parse_scenario_path","Extract the map and scenario name from a path. Crashes if the input is strange."],["path",""],["path_all_edits",""],["path_all_saves",""],["path_all_scenarios",""],["path_camera_state",""],["path_edits",""],["path_player",""],["path_popdat",""],["path_prebaked_results",""],["path_raw_map",""],["path_save",""],["path_scenario",""],["path_shared_input",""],["path_trips",""],["print_download_progress","Print download progress to STDOUT. Pass this the receiver, then call download_to_file or download_bytes with the sender."],["read_binary",""],["read_json",""],["read_object","May be a JSON or binary file"],["slurp_bytes","An adapter for widgetry::Settings::read_svg to read SVGs using this crate’s methods for finding and reading files in different environments."],["slurp_file",""],["write_binary",""],["write_json",""]],"mod":[["abst_data",""],["abst_paths","Generate paths for different A/B Street files"],["download",""],["http",""],["io",""],["io_native","Normal file IO using the filesystem"]],"struct":[["CityName","A single city is identified using this."],["DataPacks","Player-chosen groups of files to opt into downloading"],["Entry","A single file"],["FileWithProgress",""],["Manifest","A list of all canonical data files for A/B Street that’re uploaded somewhere. The file formats are tied to the latest version of the git repo. Players use the updater crate to sync these files with local copies."],["MapName","A single map is identified using this."]]});