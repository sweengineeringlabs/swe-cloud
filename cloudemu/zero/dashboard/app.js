// ZeroCloud Dashboard Logic
const API_BASE = 'http://localhost:8080';

async function fetchData(endpoint) {
    try {
        const response = await fetch(`${API_BASE}${endpoint}`);
        if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
        return await response.json();
    } catch (e) {
        console.error(`Fetch failed for ${endpoint}:`, e);
        return null;
    }
}

async function createWorkload() {
    const id = prompt("Enter Workload ID:");
    const image = prompt("Enter Image (e.g. ubuntu:latest):");
    if (!id || !image) return;

    try {
        const response = await fetch(`${API_BASE}/v1/workloads`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ id, image })
        });
        if (response.ok) {
            alert('Workload creation initiated!');
            refreshAll();
        }
    } catch (e) {
        alert('Failed to create workload: ' + e.message);
    }
}

function renderNodes(nodes) {
    const list = document.getElementById('nodes-list');
    if (!nodes || nodes.length === 0) {
        list.innerHTML = '<div class="item-sub">No nodes registered.</div>';
        return;
    }
    list.innerHTML = nodes.map(node => `
        <div class="list-item">
            <div class="item-info">
                <span class="item-name">${node.hostname}</span>
                <span class="item-sub">${node.ip_address} | ${node.status}</span>
            </div>
            <div class="btn-icon"><i data-lucide="Monitor"></i></div>
        </div>
    `).join('');
    lucide.createIcons();
}

function renderWorkloads(workloads) {
    const list = document.getElementById('workloads-list');
    // Note: Provider doesn't currently list workloads, so we might need to add a meta-storage or endpoint.
    // For now, we'll show a placeholder or mock if we haven't implemented GET /v1/workloads.
    if (!workloads || workloads.length === 0) {
        list.innerHTML = '<div class="item-sub">No active workloads.</div>';
        return;
    }
    list.innerHTML = workloads.map(w => `
        <div class="list-item">
            <div class="item-info">
                <span class="item-name">${w.id}</span>
                <span class="item-sub">${w.state} | ${w.ip_address || 'No IP'}</span>
            </div>
            <button class="btn-icon" onclick="deleteWorkload('${w.id}')"><i data-lucide="Trash2"></i></button>
        </div>
    `).join('');
    lucide.createIcons();
}

async function deleteWorkload(id) {
    if (!confirm(`Are you sure you want to delete ${id}?`)) return;
    const response = await fetch(`${API_BASE}/v1/workloads`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id })
    });
    if (response.ok) refreshAll();
}

async function refreshAll() {
    const nodes = await fetchData('/v1/nodes');
    renderNodes(nodes);

    // Placeholder: ZeroProvider doesn't have a list_workloads endpoint yet.
    // I should add one to the engine/provider.
    renderWorkloads([]);
}

// Initial Load
document.addEventListener('DOMContentLoaded', () => {
    lucide.createIcons();
    refreshAll();

    // Auto-refresh every 10 seconds
    setInterval(refreshAll, 10000);
});
