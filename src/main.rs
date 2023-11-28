use leptos::*;
mod components;
use components::*;

fn main() {
    mount_to_body(|| {
        view! {
          <>
          <div class="w-screen">
              <NavBar/>
            <main class="flex flex-col items-center gap-8">
              <Drawer/>
              <Modal/>
            </main>
          </div>
            <Footer/>
          </>
        }
    })
}
