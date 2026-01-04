import { createFileRoute } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core"

export const Route = createFileRoute("/dashboard/")({
  component: DashboardWindow,
});


function DashboardWindow() {
  const triggerWindow = async () => {
    console.log("invoking show_notification_window")
    await invoke("show_notification_window");
  }
  return (
    <main>
      <p>Dashboard</p>
      <button onClick={triggerWindow}>Trigger Notification Window</button>
    </main>
  )
}