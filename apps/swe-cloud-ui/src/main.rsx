component App {
    style {
        .app {
            min-height: 100vh;
            font-family: system-ui, sans-serif;
            background: #f8fafc;
            color: #1e293b;
        }
        .app-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 16px 32px;
            background: #ffffff;
            border-bottom: 1px solid #e2e8f0;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
        }
        .logo {
            font-size: 24px;
            font-weight: 700;
            color: #3b82f6;
        }
        .main-nav {
            display: flex;
            gap: 24px;
        }
        .nav-link {
            color: #64748b;
            text-decoration: none;
            font-weight: 500;
            transition: color 0.2s;
        }
        .nav-link:hover {
            color: #3b82f6;
        }
        .main-content {
            max-width: 1200px;
            margin: 0 auto;
            padding: 48px 32px;
        }
        .hero {
            text-align: center;
            margin-bottom: 64px;
        }
        .hero-title {
            font-size: 48px;
            font-weight: 800;
            color: #0f172a;
            margin-bottom: 16px;
        }
        .hero-subtitle {
            font-size: 20px;
            color: #64748b;
            max-width: 600px;
            margin: 0 auto;
        }
        .features {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 24px;
            margin-bottom: 64px;
        }
        .feature-card {
            background: #ffffff;
            border-radius: 12px;
            padding: 32px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
            border: 1px solid #e2e8f0;
            transition: transform 0.2s, box-shadow 0.2s;
        }
        .feature-card:hover {
            transform: translateY(-4px);
            box-shadow: 0 12px 24px rgba(0, 0, 0, 0.1);
        }
        .feature-icon {
            width: 48px;
            height: 48px;
            background: #eff6ff;
            border-radius: 10px;
            display: flex;
            align-items: center;
            justify-content: center;
            margin-bottom: 16px;
            font-size: 24px;
        }
        .feature-title {
            font-size: 20px;
            font-weight: 600;
            color: #0f172a;
            margin-bottom: 8px;
        }
        .feature-desc {
            color: #64748b;
            line-height: 1.6;
        }
        .stats {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 24px;
            margin-bottom: 64px;
        }
        .stat-card {
            background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
            border-radius: 12px;
            padding: 24px;
            color: #ffffff;
            text-align: center;
        }
        .stat-value {
            font-size: 36px;
            font-weight: 700;
            margin-bottom: 4px;
        }
        .stat-label {
            font-size: 14px;
            opacity: 0.9;
        }
        .app-footer {
            text-align: center;
            padding: 32px;
            color: #94a3b8;
            border-top: 1px solid #e2e8f0;
        }
    }
    render {
        <div class="app">
            <header class="app-header">
                <div class="logo">"SWE Cloud"</div>
                <nav class="main-nav">
                    <a href="/" class="nav-link">"Dashboard"</a>
                    <a href="/projects" class="nav-link">"Projects"</a>
                    <a href="/deploy" class="nav-link">"Deploy"</a>
                    <a href="/settings" class="nav-link">"Settings"</a>
                </nav>
            </header>
            <main class="main-content">
                <section class="hero">
                    <h1 class="hero-title">"Software Engineering Cloud"</h1>
                    <p class="hero-subtitle">"Build, deploy, and scale your applications with the power of RustScript and WebAssembly"</p>
                </section>
                <section class="features">
                    <div class="feature-card">
                        <div class="feature-icon">"⚡"</div>
                        <h3 class="feature-title">"Lightning Fast"</h3>
                        <p class="feature-desc">"WebAssembly compilation delivers near-native performance in the browser"</p>
                    </div>
                    <div class="feature-card">
                        <div class="feature-icon">"🔒"</div>
                        <h3 class="feature-title">"Type Safe"</h3>
                        <p class="feature-desc">"Catch errors at compile time with RustScript's powerful type system"</p>
                    </div>
                    <div class="feature-card">
                        <div class="feature-icon">"🚀"</div>
                        <h3 class="feature-title">"Easy Deploy"</h3>
                        <p class="feature-desc">"One-click deployment to global edge networks with instant scaling"</p>
                    </div>
                </section>
                <section class="stats">
                    <div class="stat-card">
                        <div class="stat-value">"2.5ms"</div>
                        <div class="stat-label">"Avg Response Time"</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">"99.9%"</div>
                        <div class="stat-label">"Uptime SLA"</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">"150+"</div>
                        <div class="stat-label">"Edge Locations"</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">"10K+"</div>
                        <div class="stat-label">"Active Projects"</div>
                    </div>
                </section>
            </main>
            <footer class="app-footer">
                <p>"Built with RustScript • Powered by WebAssembly"</p>
            </footer>
        </div>
    }
}
