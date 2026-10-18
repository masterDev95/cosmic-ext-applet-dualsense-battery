name := "cosmic-ext-applet-dualsense-battery"
app_id := "io.github.axolotl.CosmicDualsenseBattery"
user_prefix := env_var("HOME") + "/.local"

build:
    cargo build --release

install-user: build
    install -Dm0755 "target/release/{{name}}" "{{user_prefix}}/bin/{{name}}"
    install -Dm0644 "resources/{{app_id}}.desktop" "{{user_prefix}}/share/applications/{{app_id}}.desktop"

uninstall-user:
    rm -f "{{user_prefix}}/bin/{{name}}"
    rm -f "{{user_prefix}}/share/applications/{{app_id}}.desktop"

test:
    cargo test

fmt:
    cargo fmt --all
