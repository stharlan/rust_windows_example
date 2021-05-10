fn main() {
    windows::build!(
        Windows::Web::Syndication::*,
        Windows::Foundation::*,
        Windows::Foundation::Collections::*,
        Windows::Win32::WindowsAndMessaging::*,
        Windows::Win32::SystemServices::*,
        Windows::Win32::MenusAndResources::*,
        Windows::Win32::Gdi::*,
    );
}