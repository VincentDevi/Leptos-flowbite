use leptos::*;
mod components;
use components::*;

fn main() {
    mount_to_body(|| {
        view! {
          <>
          <Drawer/>
          <Modal/>
          </>
        }
    })
}
