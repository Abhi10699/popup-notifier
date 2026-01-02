import "./App.css";
import { invoke } from "@tauri-apps/api/core";

function App() {
  return (
    <main className="container">
      <div className="card">
        <div className="icon">💧</div>

        <div className="content">
          <h2>Hydration Check</h2>
          <p>Time to drink a glass of water.</p>
        </div>

        <div className="actions">
          <button
            className="primary"
            onClick={() => invoke("hide_window")}
          >
            I drank water
          </button>

          <button
            className="secondary"
            onClick={() => invoke("hide_window")}
          >
            Remind me later
          </button>
        </div>
      </div>
    </main>
  );
}

export default App;