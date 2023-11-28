use leptos::{component, view, IntoView};

#[component]
pub fn Drawer() -> impl IntoView {
    view! {
      <>
        <div>
          <button
            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800"
            type="button"
            data-drawer-target="drawer-right-example"
            data-drawer-show="drawer-right-example"
            data-drawer-placement="right"
            aria-controls="drawer-right-example"
          >
            Show right drawer
          </button>
        </div>

        <div
          id="drawer-right-example"
          class="fixed top-0 right-0 z-40 h-screen p-4 overflow-y-auto transition-transform translate-x-full bg-white w-80 dark:bg-gray-800"
          tabindex="-1"
          aria-labelledby="drawer-right-label"
        >
          <h5
            id="drawer-right-label"
            class="inline-flex items-center mb-4 text-base font-semibold text-gray-500 dark:text-gray-400"
          >
            Right drawer
          </h5>
        </div>
      </>
    }
}
