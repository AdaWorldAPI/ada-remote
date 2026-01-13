import { invoke } from '@tauri-apps/api/tauri';
import './style.css';

// Tab switching
document.querySelectorAll('.tab-button').forEach(button => {
  button.addEventListener('click', () => {
    const targetTab = button.dataset.tab;

    document.querySelectorAll('.tab-button').forEach(btn => btn.classList.remove('active'));
    document.querySelectorAll('.tab-content').forEach(content => content.classList.remove('active'));

    button.classList.add('active');
    document.getElementById(`${targetTab}-tab`).classList.add('active');
  });
});

// Start hosting session
document.getElementById('start-host-btn').addEventListener('click', async () => {
  const password = document.getElementById('host-password').value;
  const statusText = document.getElementById('status-text');

  try {
    statusText.textContent = 'Starting session...';

    const sessionInfo = await invoke('start_host_session', {
      password: password || null,
    });

    document.getElementById('session-id').textContent = sessionInfo.session_id;
    document.getElementById('session-info').style.display = 'block';
    document.getElementById('start-host-btn').style.display = 'none';

    statusText.textContent = 'Session active - Waiting for connection';
  } catch (error) {
    statusText.textContent = `Error: ${error}`;
    console.error(error);
  }
});

// Connect to session
document.getElementById('connect-btn').addEventListener('click', async () => {
  const sessionId = document.getElementById('client-session-id').value;
  const password = document.getElementById('client-password').value;
  const statusText = document.getElementById('status-text');

  if (!sessionId) {
    statusText.textContent = 'Please enter a session ID';
    return;
  }

  try {
    statusText.textContent = 'Connecting...';

    const sessionInfo = await invoke('connect_to_session', {
      sessionId,
      password: password || null,
    });

    statusText.textContent = `Connected to session ${sessionInfo.session_id}`;
  } catch (error) {
    statusText.textContent = `Connection failed: ${error}`;
    console.error(error);
  }
});

// Disconnect session
document.getElementById('disconnect-btn').addEventListener('click', async () => {
  const statusText = document.getElementById('status-text');

  try {
    await invoke('disconnect_session');

    document.getElementById('session-info').style.display = 'none';
    document.getElementById('start-host-btn').style.display = 'block';
    document.getElementById('host-password').value = '';

    statusText.textContent = 'Disconnected';
  } catch (error) {
    statusText.textContent = `Error: ${error}`;
    console.error(error);
  }
});
