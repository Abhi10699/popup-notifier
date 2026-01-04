import { createFileRoute } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";


export const Route = createFileRoute('/notification/')({
  component: NotificationWindow
})


function NotificationWindow() {
  return (
    <div className="flex h-screen w-screen items-center justify-center bg-transparent">
      <div className="w-full rounded-xl bg-white p-5 shadow-lg">
        {/* Icon */}
        <div className="mb-3 flex justify-center">
          <div className="flex h-12 w-12 items-center justify-center rounded-full bg-blue-100 text-2xl">
            💧
          </div>
        </div>

        {/* Content */}
        <div className="text-center">
          <h2 className="text-lg font-semibold text-gray-900">
            Hydration Check
          </h2>
          <p className="mt-1 text-sm text-gray-600">
            Time to drink a glass of water.
          </p>
        </div>

        {/* Actions */}
        <div className="mt-5 flex gap-2">
          <button
            onClick={() => invoke("hide_window")}
            className="flex-1 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition hover:bg-blue-700 active:scale-[0.98]"
          >
            I drank water
          </button>

          <button
            onClick={() => invoke("hide_window")}
            className="flex-1 rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 transition hover:bg-gray-100 active:scale-[0.98]"
          >
            Remind me later
          </button>
        </div>
      </div>
    </div>
  );
}
