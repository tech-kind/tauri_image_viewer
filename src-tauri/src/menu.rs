use json_gettext::{get_text, static_json_gettext_build};
use std::env::consts;
use sys_locale::get_locale;
use tauri::utils::assets::EmbeddedAssets;
use tauri::{Context, CustomMenuItem, Menu, MenuItem, Submenu};

fn get_close_accelerator() -> String {
  if consts::OS == "macos" {
    "Cmd+W".to_string()
  } else {
    "Alt+F4".to_string()
  }
}

fn get_fullscreen_accelerator() -> String {
  if consts::OS == "macos" {
    "Cmd+Ctrl+F".to_string()
  } else {
    "F11".to_string()
  }
}

pub fn default(_app_context: &Context<EmbeddedAssets>) -> Menu {
  let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
  let ctx = static_json_gettext_build!(
    "en-US";
    "en-US" => "locales/en.json",
    "ja-JP" => "locales/ja.json",
  )
  .unwrap();

  #[cfg(target_os = "macos")]
  let app_menu = Submenu::new(
    &_app_context.package_info().name,
    Menu::new()
      .add_native_item(MenuItem::About(
        _app_context.package_info().name.clone(),
        tauri::AboutMetadata::new(),
      ))
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Hide)
      .add_native_item(MenuItem::HideOthers)
      .add_native_item(MenuItem::ShowAll)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Quit)
  );

  let file_menu = Submenu::new(
    get_text!(ctx, &locale, "File").unwrap().to_string(),
    Menu::new()
      .add_item(
        CustomMenuItem::new(
          "open", 
          get_text!(ctx, &locale, "Open...")
            .unwrap()
            .to_string(),
        )
        .accelerator("CmdOrCtrl+O")
      )
      .add_native_item(MenuItem::Separator)
      .add_item(
        CustomMenuItem::new(
          "remove", 
          get_text!(ctx, &locale, "Move to Trash")
            .unwrap()
            .to_string(),
        )
        .accelerator("DELETE")
      )
      .add_native_item(MenuItem::Separator)
      .add_item(
        CustomMenuItem::new(
          "close", 
          get_text!(ctx, &locale, "Close")
            .unwrap()
            .to_string(),
        )
        .accelerator(get_close_accelerator())
      ),
  );

  let view_menu = Submenu::new(
    get_text!(ctx, &locale, "View").unwrap().to_string(),
    Menu::new()
      .add_item(
        CustomMenuItem::new(
          "next", 
          get_text!(ctx, &locale, "Next Image")
            .unwrap()
            .to_string(),
        )
        .accelerator("J")
      )
      .add_item(
        CustomMenuItem::new(
          "prev", 
          get_text!(ctx, &locale, "Prev Image")
            .unwrap()
            .to_string(),
        )
        .accelerator("K")
      )
      .add_native_item(MenuItem::Separator)
      .add_item(
        CustomMenuItem::new(
          "grid", 
          get_text!(ctx, &locale, "Toggle Grid View")
            .unwrap()
            .to_string(),
        )
        .accelerator("H")
      ),
  );

  let window_menu = Submenu::new(
    get_text!(ctx, &locale, "Window").unwrap().to_string(),
    Menu::new()
      .add_item(
        CustomMenuItem::new(
          "minimize", 
          get_text!(ctx, &locale, "Minimize")
            .unwrap()
            .to_string(),
        )
        .accelerator("CmdOrCtrl+M")
      )
      .add_item(
        CustomMenuItem::new(
          "zoom", 
          get_text!(ctx, &locale, "Zoom")
            .unwrap()
            .to_string(),
        )
      )
      .add_native_item(MenuItem::Separator)
      .add_item(
        CustomMenuItem::new(
          "fullscreen", 
          get_text!(ctx, &locale, "Toggle Fullscreen")
            .unwrap()
            .to_string(),
        )
        .accelerator(get_fullscreen_accelerator())
      ),
  );

  let help_menu = Submenu::new(
    get_text!(ctx, &locale, "Help").unwrap().to_string(),
    Menu::new()
      .add_item(
        CustomMenuItem::new(
          "Support", 
          get_text!(ctx, &locale, "Support URL...")
            .unwrap()
            .to_string(),
        )
      ),
  );

  #[cfg(target_os = "macos")]
  let menu = Menu::new()
    .add_submenu(app_menu)
    .add_submenu(file_menu)
    .add_submenu(view_menu)
    .add_submenu(window_menu)
    .add_submenu(help_menu);

  #[cfg(not(target_os = "macos"))]
  let menu = Menu::new()
    .add_submenu(file_menu)
    .add_submenu(view_menu)
    .add_submenu(window_menu)
    .add_submenu(help_menu);

  menu
}