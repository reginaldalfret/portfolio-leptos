use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Interactive,
    Pdf,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PrCategory {
    All,
    BigTech,
    AiMl,
    CloudInfra,
}

struct Contribution {
    org: &'static str,
    repo: &'static str,
    pr: &'static str,
    url: &'static str,
    category: PrCategory,
    highlight: &'static str,
}

struct Experience {
    company: &'static str,
    role: &'static str,
    period: &'static str,
    location: &'static str,
    bullets: &'static [&'static str],
}

struct Project {
    title: &'static str,
    badge: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    bullets: &'static [&'static str],
}

struct Achievement {
    icon: &'static str,
    title: &'static str,
    subtitle: &'static str,
}

struct Certification {
    icon: &'static str,
    title: &'static str,
    issuer: &'static str,
}

struct StackItem {
    name: &'static str,
    href: &'static str,
    badge: &'static str,
    icon: &'static str,
    desc: &'static str,
    is_active: bool,
}

const CONTRIBUTIONS: &[Contribution] = &[
    Contribution { org: "Apple", repo: "swift-numerics", pr: "#355", url: "https://github.com/apple/swift-numerics/pull/355", category: PrCategory::BigTech, highlight: "Complex Trigonometric Functions" },
    Contribution { org: "Google", repo: "Highway", pr: "#3511", url: "https://github.com/google/highway/pull/3511", category: PrCategory::BigTech, highlight: "SIMD Highway Acceleration" },
    Contribution { org: "Google Cloud", repo: "Data Validator", pr: "#1863", url: "https://github.com/GoogleCloudPlatform/professional-services-data-validator/pull/1863", category: PrCategory::CloudInfra, highlight: "BigQuery BigNumeric validation" },
    Contribution { org: "Microsoft", repo: "Semantic Kernel", pr: "#14472", url: "https://github.com/microsoft/semantic-kernel/pull/14472", category: PrCategory::AiMl, highlight: "Agent prompt filter fixes" },
    Contribution { org: "AWS", repo: "Lambda Powertools", pr: "#8542", url: "https://github.com/aws-powertools/powertools-lambda-python/pull/8542", category: PrCategory::CloudInfra, highlight: "Event handler validation" },
    Contribution { org: "Meta", repo: "StyleX", pr: "#1899", url: "https://github.com/facebook/stylex/pull/1899", category: PrCategory::BigTech, highlight: "CSS build compiler fixes" },
    Contribution { org: "NVIDIA", repo: "cuOpt", pr: "#2068", url: "https://github.com/NVIDIA/cuopt/pull/2068", category: PrCategory::AiMl, highlight: "GPU optimization routing" },
    Contribution { org: "NASA", repo: "ProgPy", pr: "#237", url: "https://github.com/nasa/prog_models/pull/237", category: PrCategory::AiMl, highlight: "Prognostics physics models" },
    Contribution { org: "Azure", repo: "Cosmos DB Emulator", pr: "#359", url: "https://github.com/Azure/azure-cosmos-db-emulator-docker/pull/359", category: PrCategory::CloudInfra, highlight: "Docker Linux container runner" },
    Contribution { org: "Adobe", repo: "Skills", pr: "#445", url: "https://github.com/adobe/skills/pull/445", category: PrCategory::BigTech, highlight: "Workflow orchestration types" },
    Contribution { org: "Intel", repo: "AutoRound", pr: "#2421", url: "https://github.com/intel/auto-round/pull/2421", category: PrCategory::AiMl, highlight: "Low-bit LLM quantization" },
    Contribution { org: "IBM", repo: "Docling Pipelines", pr: "#85", url: "https://github.com/IBM/docling-pipelines/pull/85", category: PrCategory::AiMl, highlight: "Document parsing pipeline" },
    Contribution { org: "Uber", repo: "Kraken", pr: "#716", url: "https://github.com/uber/kraken/pull/716", category: PrCategory::CloudInfra, highlight: "P2P Docker registry engine" },
    Contribution { org: "Airbnb", repo: "MagazineLayout", pr: "#169", url: "https://github.com/airbnb/MagazineLayout/pull/169", category: PrCategory::BigTech, highlight: "UICollectionView grid layout" },
    Contribution { org: "GitHub", repo: "GitHub CLI", pr: "#14592", url: "https://github.com/cli/cli/pull/14592", category: PrCategory::BigTech, highlight: "GraphQL alias queries" },
    Contribution { org: "Dell", repo: "Omnia Containers", pr: "#649", url: "https://github.com/dell/omnia/pull/649", category: PrCategory::CloudInfra, highlight: "HPC Slurm deployment" },
    Contribution { org: "Salesforce", repo: "tough-cookie", pr: "#659", url: "https://github.com/salesforce/tough-cookie/pull/659", category: PrCategory::CloudInfra, highlight: "Cookie parser security RFC6265" },
    Contribution { org: "AMD", repo: "ROCm HRX System", pr: "#1231", url: "https://github.com/ROCm/hrx-system/pull/1231", category: PrCategory::AiMl, highlight: "GPU telemetry monitoring" },
    Contribution { org: "SAP", repo: "E-Mobility Simulator", pr: "#2136", url: "https://github.com/SAP/e-mobility-charging-stations-simulator/pull/2136", category: PrCategory::CloudInfra, highlight: "OCPP 2.0.1 charge station logic" },
    Contribution { org: "Pinterest", repo: "PyMemcache", pr: "#657", url: "https://github.com/pinterest/pymemcache/pull/657", category: PrCategory::CloudInfra, highlight: "Connection pooling failover" },
    Contribution { org: "Qualcomm", repo: "Commit Msg Check", pr: "#38", url: "https://github.com/qualcomm/commit-msg-check-action/pull/38", category: PrCategory::BigTech, highlight: "CI/CD git validation" },
    Contribution { org: "Stripe", repo: "Stripe React Native", pr: "#2751", url: "https://github.com/stripe/stripe-react-native/pull/2751", category: PrCategory::BigTech, highlight: "PaymentSheet callback handlers" },
    Contribution { org: "Docker", repo: "Docker Docs", pr: "#26266", url: "https://github.com/docker/docs/pull/26266", category: PrCategory::CloudInfra, highlight: "Compose V2 multi-stage configs" },
    Contribution { org: "Apache", repo: "HertzBeat", pr: "#4410", url: "https://github.com/apache/hertzbeat/pull/4410", category: PrCategory::CloudInfra, highlight: "Real-time network telemetry" },
    Contribution { org: "Cloudflare", repo: "CFSSL", pr: "#1451", url: "https://github.com/cloudflare/cfssl/pull/1451", category: PrCategory::CloudInfra, highlight: "TLS certificate bundler CLI" },
    Contribution { org: "OpenAI", repo: "Softnet", pr: "#219", url: "https://github.com/openai/softnet/pull/219", category: PrCategory::AiMl, highlight: "Gradient descent scheduler" },
    Contribution { org: "Snapchat", repo: "ts-inject", pr: "#30", url: "https://github.com/Snapchat/ts-inject/pull/30", category: PrCategory::BigTech, highlight: "Dependency injection typings" },
    Contribution { org: "Spotify", repo: "Confidence SDK", pr: "#272", url: "https://github.com/spotify/confidence-sdk-swift/pull/272", category: PrCategory::BigTech, highlight: "Feature flagging Swift SDK" },
    Contribution { org: "Cisco", repo: "cJOSE", pr: "#202", url: "https://github.com/cisco/cjose/pull/202", category: PrCategory::CloudInfra, highlight: "JWE encryption RFC7516" },
    Contribution { org: "Netflix", repo: "Atlas", pr: "#2011", url: "https://github.com/Netflix/atlas/pull/2011", category: PrCategory::CloudInfra, highlight: "In-memory time series metrics" },
];

const EXPERIENCES: &[Experience] = &[
    Experience {
        company: "DailyGurus",
        role: "Software Developer",
        period: "Jul 2026 – Present",
        location: "Chennai, India",
        bullets: &[
            "Architecting enterprise ERP solutions using React, Node.js, and TypeScript with high-concurrency microservices.",
            "Spearheading module customizations and workflow automations in ERPNext for mission-critical operations.",
            "Building low-latency REST and GraphQL APIs backed by PostgreSQL and Redis caches.",
        ],
    },
    Experience {
        company: "Data Patterns",
        role: "Data Science Intern",
        period: "May 2025 – Jun 2025",
        location: "Chennai, India",
        bullets: &[
            "Engineered high-throughput automated scrapers to curate domain-specific aerospace research datasets from IEEE.",
            "Fine-tuned open-source LLMs using LoRA (Low-Rank Adaptation) and QLoRA for domain-adapted technical QA.",
            "Evaluated model inference speed and perplexity, achieving 35% reduction in latency via vLLM quantization.",
        ],
    },
];

const PROJECTS: &[Project] = &[
    Project {
        title: "301 Super AI Agents — Sovereign Multi-Agent AI Platform",
        badge: "🥇 1st Prize · HackIndia 2026",
        description: "Local-first autonomous multi-agent platform orchestrating 301 specialized AI agents for real-world workflow automation.",
        tags: &["Rust", "Leptos WASM", "FastAPI", "Ollama LLMs", "OCR", "Multi-Agent System"],
        bullets: &[
            "Designed a hierarchical agent swarm coordinated by central orchestrator Agent 301.",
            "Implemented offline local inference with Ollama, multimodal OCR extraction, and real-time streaming.",
            "Engineered zero-telemetry sovereign execution designed for strict enterprise security compliance.",
        ],
    },
    Project {
        title: "TOR-UNVEIL — Dark Web AI Cybercriminal Tracker",
        badge: "🥇 1st Prize · TN Police Hackathon 2025",
        description: "Full-stack AI surveillance platform adapted for live cybercriminal intelligence by the Tamil Nadu Police.",
        tags: &["Python", "Network Analysis", "Graph Neural Nets", "TypeScript", "Tailwind CSS"],
        bullets: &[
            "Built automated crawling and onion-service profiling engine for illicit darknet marketplace monitoring.",
            "Developed entity resolution, bitcoin transaction correlation, and dynamic crime network graph visualization.",
        ],
    },
    Project {
        title: "AI Crop Recommendation & Agri-Intelligence Engine",
        badge: "🥇 1st Prize · Blaze a Trail 2.0",
        description: "Precision agriculture system utilizing satellite GIS data, multi-spectral imagery, and ML classifiers.",
        tags: &["Python", "Scikit-learn", "GIS Data", "Explainable AI", "REST API"],
        bullets: &[
            "Analyzed soil NPK, humidity, precipitation, and satellite indices to recommend optimal crops with 96% accuracy.",
            "Integrated SHAP & LIME for Explainable AI predictions to build trust with local agrarian communities.",
        ],
    },
];

const ACHIEVEMENTS: &[Achievement] = &[
    Achievement { icon: "🥇", title: "1st Prize – HackIndia 2026", subtitle: "301 Super AI Agents (500+ Teams)" },
    Achievement { icon: "🥇", title: "1st Prize – TN Police Hackathon 2025", subtitle: "TOR-UNVEIL Dark Web Tracker" },
    Achievement { icon: "🥇", title: "1st Prize – Blaze a Trail 2.0", subtitle: "Crop Recommendation Engine" },
    Achievement { icon: "🛡️", title: "State Finalist – TN Police Hackathon 2024", subtitle: "Automated Cybersecurity Threat Analyzer" },
    Achievement { icon: "🏆", title: "Winner – 6 National Technical Symposiums", subtitle: "Autonomous Systems & Coding Challenges" },
    Achievement { icon: "🚀", title: "70+ Production Projects", subtitle: "Across AI, Systems, WebAssembly & Full-Stack" },
];

const CERTIFICATIONS: &[Certification] = &[
    Certification { icon: "🎓", title: "Meta Front-End Developer Professional", issuer: "Coursera / Meta" },
    Certification { icon: "🛡️", title: "Google Cybersecurity Professional", issuer: "Coursera / Google" },
    Certification { icon: "⚛️", title: "React: The Complete Guide", issuer: "Udemy" },
    Certification { icon: "🎨", title: "Figma UI/UX Design Essentials", issuer: "Udemy" },
];

const STACKS: &[StackItem] = &[
    StackItem { name: "Next.js", href: "/", badge: "Core Engine", icon: "▲", desc: "Next.js 14 App Router · React 18 · TypeScript", is_active: false },
    StackItem { name: "Nuxt 3", href: "/stacks/nuxt", badge: "Live SSR", icon: "▲", desc: "Nuxt 3 · Vue 3 Composition API · Nitro", is_active: false },
    StackItem { name: "Django", href: "/stacks/django", badge: "Live SSR", icon: "dj", desc: "Django 6 · Python 3.13 · Jinja Templates", is_active: false },
    StackItem { name: "Spring Boot", href: "/stacks/springboot", badge: "Live SSR", icon: "🍃", desc: "Spring Boot 3 · Java 21 · Thymeleaf", is_active: false },
    StackItem { name: "Laravel", href: "/stacks/laravel", badge: "Live SSR", icon: "🔴", desc: "Laravel 11 · PHP 8.4 · Blade SSR", is_active: false },
    StackItem { name: "Rails", href: "/stacks/rails", badge: "Live SSR", icon: "💎", desc: "Ruby on Rails 8 · Ruby 3.4 · ERB", is_active: false },
    StackItem { name: "Angular", href: "/stacks/angular", badge: "Live SPA", icon: "🅰️", desc: "Angular 19/22 · TypeScript 6.0 · Signals", is_active: false },
    StackItem { name: "Blazor", href: "/stacks/blazor", badge: "Live WASM", icon: "🟣", desc: "Blazor WebAssembly · .NET 8 · C#", is_active: false },
    StackItem { name: "Buffalo", href: "/stacks/buffalo", badge: "Live Go", icon: "🦬", desc: "Go Buffalo · Go 1.27 · Plush SSR", is_active: false },
    StackItem { name: "Leptos", href: "/stacks/leptos", badge: "Active (Rust)", icon: "🦀", desc: "Leptos 0.7 · Rust WebAssembly · CSR", is_active: true },
    StackItem { name: "Jaspr", href: "/stacks/jaspr", badge: "Live Dart", icon: "🎯", desc: "Dart 3.13 · Jaspr 0.23 · SSR Hydration", is_active: false },
];

#[component]
fn App() -> impl IntoView {
    let (view_mode, set_view_mode) = signal(ViewMode::Interactive);
    let (pr_filter, set_pr_filter) = signal(PrCategory::All);

    let filtered_contributions = move || {
        let filter = pr_filter.get();
        CONTRIBUTIONS
            .iter()
            .filter(move |c| match filter {
                PrCategory::All => true,
                _ => c.category == filter,
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="min-h-screen bg-[#050505] text-neutral-100 relative overflow-x-hidden selection:bg-orange-500/20 selection:text-orange-200">
            // Ambient Background Lighting
            <div class="fixed inset-0 pointer-events-none overflow-hidden z-0">
                <div class="absolute -top-40 -left-40 w-96 h-96 bg-orange-600/15 rounded-full blur-[140px]"></div>
                <div class="absolute top-1/3 -right-40 w-96 h-96 bg-red-600/10 rounded-full blur-[140px]"></div>
                <div class="absolute -bottom-40 left-1/3 w-96 h-96 bg-amber-600/10 rounded-full blur-[140px]"></div>
            </div>

            // Main Container
            <main class="relative z-10 max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8 sm:py-12 space-y-12">
                
                // Top Command Bar
                <header class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 p-4 rounded-3xl bg-neutral-900/60 border border-white/10 backdrop-blur-xl">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 rounded-2xl bg-gradient-to-tr from-orange-600 via-red-500 to-amber-500 flex items-center justify-center font-black text-xl text-black shadow-lg shadow-orange-500/20">
                            "🦀"
                        </div>
                        <div>
                            <div class="flex items-center gap-2">
                                <span class="font-bold text-white text-base">"Reginald Alfret V"</span>
                                <span class="text-[10px] font-mono px-2 py-0.5 rounded-full bg-orange-500/20 text-orange-300 border border-orange-500/40 flex items-center gap-1 font-semibold">
                                    <span class="w-1.5 h-1.5 rounded-full bg-orange-400 animate-pulse"></span>
                                    "LEPTOS · RUST WASM"
                                </span>
                            </div>
                            <p class="text-xs text-neutral-400 font-mono">"Open-Source Contributor · AI & Data Science Engineer"</p>
                        </div>
                    </div>

                    <div class="flex items-center gap-2 w-full sm:w-auto justify-end">
                        <button
                            type="button"
                            on:click=move |_| set_view_mode.update(|m| *m = match *m {
                                ViewMode::Interactive => ViewMode::Pdf,
                                ViewMode::Pdf => ViewMode::Interactive,
                            })
                            class="px-3.5 py-1.5 rounded-xl text-xs font-semibold bg-white/10 hover:bg-white/15 text-white transition-all flex items-center gap-1.5 border border-white/10"
                        >
                            {move || match view_mode.get() {
                                ViewMode::Interactive => "Original Resume (PDF)",
                                ViewMode::Pdf => "Interactive Resume",
                            }}
                        </button>
                        <a
                            href="/Reginald_Alfret_Resume.pdf"
                            download
                            class="px-3.5 py-1.5 rounded-xl text-xs font-semibold bg-orange-500/20 hover:bg-orange-500/30 text-orange-300 transition-all flex items-center gap-1.5 border border-orange-500/40 shadow-sm"
                        >
                            "Download PDF"
                        </a>
                    </div>
                </header>

                // Conditional View: PDF Document vs Interactive Resume
                {move || match view_mode.get() {
                    ViewMode::Pdf => view! {
                        <div class="rounded-3xl border border-white/15 bg-neutral-900/80 p-4 backdrop-blur-xl shadow-2xl space-y-4">
                            <div class="flex items-center justify-between pb-3 border-b border-white/10">
                                <div class="flex items-center gap-2">
                                    <span class="text-sm font-bold text-white">"Reginald Alfret V — Official Resume"</span>
                                    <span class="text-xs font-mono text-neutral-400">"(PDF Document Viewer)"</span>
                                </div>
                                <button
                                    type="button"
                                    on:click=move |_| set_view_mode.set(ViewMode::Interactive)
                                    class="px-3 py-1 rounded-lg bg-white/10 text-white text-xs font-semibold hover:bg-white/20 transition-colors"
                                >
                                    "← Back to Interactive"
                                </button>
                            </div>
                            <div class="w-full h-[85vh] rounded-2xl overflow-hidden border border-white/10 bg-black">
                                <iframe src="/Reginald_Alfret_Resume.pdf" class="w-full h-full border-0" title="Resume PDF"></iframe>
                            </div>
                        </div>
                    }.into_any(),

                    ViewMode::Interactive => view! {
                        <div class="space-y-12">
                            // Hero & Bio
                            <section class="space-y-6">
                                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-neutral-900 border border-white/10 text-xs text-neutral-300 font-mono">
                                    <span class="w-2 h-2 rounded-full bg-emerald-400 animate-ping"></span>
                                    "Open to Global Software, AI & Systems Opportunities"
                                </div>
                                <div class="space-y-3">
                                    <h1 class="text-3xl sm:text-5xl font-black tracking-tight text-white">
                                        "Architecting High-Performance Systems with "
                                        <span class="bg-gradient-to-r from-orange-400 via-red-400 to-amber-300 bg-clip-text text-transparent">
                                            "Rust, Leptos & AI"
                                        </span>
                                    </h1>
                                    <p class="text-neutral-400 text-sm sm:text-base leading-relaxed max-w-3xl">
                                        "Software Developer and AI Engineer with "
                                        <strong class="text-white font-semibold">"30 merged pull requests"</strong>
                                        " across the world's most critical open-source repositories including Apple, Google, Microsoft, Meta, AWS, NVIDIA, NASA, and Netflix. 1st Prize winner in 3 national & state hackathons."
                                    </p>
                                </div>

                                // Metrics Grid
                                <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 sm:gap-4 pt-2">
                                    <div class="p-4 rounded-2xl bg-neutral-900/50 border border-white/10 backdrop-blur-sm">
                                        <div class="text-2xl sm:text-3xl font-black text-orange-400">"30"</div>
                                        <div class="text-xs font-mono text-neutral-400 mt-1">"Merged PRs in Big Tech"</div>
                                    </div>
                                    <div class="p-4 rounded-2xl bg-neutral-900/50 border border-white/10 backdrop-blur-sm">
                                        <div class="text-2xl sm:text-3xl font-black text-red-400">"10"</div>
                                        <div class="text-xs font-mono text-neutral-400 mt-1">"Full Stacks Available"</div>
                                    </div>
                                    <div class="p-4 rounded-2xl bg-neutral-900/50 border border-white/10 backdrop-blur-sm">
                                        <div class="text-2xl sm:text-3xl font-black text-amber-400">"3x"</div>
                                        <div class="text-xs font-mono text-neutral-400 mt-1">"1st Prize Hackathons"</div>
                                    </div>
                                    <div class="p-4 rounded-2xl bg-neutral-900/50 border border-white/10 backdrop-blur-sm">
                                        <div class="text-2xl sm:text-3xl font-black text-emerald-400">"70+"</div>
                                        <div class="text-xs font-mono text-neutral-400 mt-1">"Shipped Projects"</div>
                                    </div>
                                </div>
                            </section>

                            // 10-Stack Architecture Showcase
                            <section class="space-y-4">
                                <div class="flex items-center justify-between pb-2 border-b border-white/10">
                                    <div class="flex items-center gap-2">
                                        <span class="text-sm font-bold uppercase tracking-wider text-orange-400 font-mono">"// Architecture Matrix"</span>
                                        <h2 class="text-lg font-bold text-white">"10 Full-Stack Implementations"</h2>
                                    </div>
                                    <span class="text-xs font-mono text-neutral-500">"100% Production Ready"</span>
                                </div>
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-3">
                                    {STACKS.iter().map(|item| {
                                        view! {
                                            <a
                                                href=item.href
                                                target="_top"
                                                class=format!(
                                                    "p-3.5 rounded-2xl border transition-all flex flex-col justify-between gap-2 group {}",
                                                    if item.is_active {
                                                        "bg-orange-950/30 border-orange-500/50 shadow-lg shadow-orange-500/10 ring-1 ring-orange-500/30"
                                                    } else {
                                                        "bg-neutral-900/40 border-white/10 hover:border-white/20 hover:bg-neutral-900/70"
                                                    }
                                                )
                                            >
                                                <div class="flex items-center justify-between">
                                                    <span class="text-xl">{item.icon}</span>
                                                    <span class=format!(
                                                        "text-[9px] font-mono px-2 py-0.5 rounded-full uppercase tracking-wider font-bold {}",
                                                        if item.is_active {
                                                            "bg-orange-500/20 text-orange-300 border border-orange-500/40"
                                                        } else {
                                                            "bg-white/5 text-neutral-400 border border-white/10 group-hover:text-white"
                                                        }
                                                    )>
                                                        {item.badge}
                                                    </span>
                                                </div>
                                                <div>
                                                    <div class="font-bold text-sm text-white group-hover:text-orange-300 transition-colors">
                                                        {item.name}
                                                    </div>
                                                    <div class="text-[11px] text-neutral-400 font-mono mt-0.5 leading-tight">
                                                        {item.desc}
                                                    </div>
                                                </div>
                                            </a>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>

                            // Open Source Contributions (30 Merged PRs)
                            <section class="space-y-4">
                                <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-2 border-b border-white/10">
                                    <div class="flex items-center gap-2">
                                        <span class="text-sm font-bold uppercase tracking-wider text-orange-400 font-mono">"// Proven Impact"</span>
                                        <h2 class="text-lg font-bold text-white">"30 Merged Open-Source PRs"</h2>
                                    </div>
                                    <div class="flex flex-wrap items-center gap-1.5 text-xs font-mono">
                                        <button
                                            type="button"
                                            on:click=move |_| set_pr_filter.set(PrCategory::All)
                                            class=move || format!(
                                                "px-2.5 py-1 rounded-lg transition-all {}",
                                                if pr_filter.get() == PrCategory::All {
                                                    "bg-orange-500/20 text-orange-300 border border-orange-500/40 font-semibold"
                                                } else {
                                                    "bg-white/5 text-neutral-400 hover:text-white"
                                                }
                                            )
                                        >
                                            "All (30)"
                                        </button>
                                        <button
                                            type="button"
                                            on:click=move |_| set_pr_filter.set(PrCategory::BigTech)
                                            class=move || format!(
                                                "px-2.5 py-1 rounded-lg transition-all {}",
                                                if pr_filter.get() == PrCategory::BigTech {
                                                    "bg-orange-500/20 text-orange-300 border border-orange-500/40 font-semibold"
                                                } else {
                                                    "bg-white/5 text-neutral-400 hover:text-white"
                                                }
                                            )
                                        >
                                            "Big Tech"
                                        </button>
                                        <button
                                            type="button"
                                            on:click=move |_| set_pr_filter.set(PrCategory::AiMl)
                                            class=move || format!(
                                                "px-2.5 py-1 rounded-lg transition-all {}",
                                                if pr_filter.get() == PrCategory::AiMl {
                                                    "bg-orange-500/20 text-orange-300 border border-orange-500/40 font-semibold"
                                                } else {
                                                    "bg-white/5 text-neutral-400 hover:text-white"
                                                }
                                            )
                                        >
                                            "AI / ML"
                                        </button>
                                        <button
                                            type="button"
                                            on:click=move |_| set_pr_filter.set(PrCategory::CloudInfra)
                                            class=move || format!(
                                                "px-2.5 py-1 rounded-lg transition-all {}",
                                                if pr_filter.get() == PrCategory::CloudInfra {
                                                    "bg-orange-500/20 text-orange-300 border border-orange-500/40 font-semibold"
                                                } else {
                                                    "bg-white/5 text-neutral-400 hover:text-white"
                                                }
                                            )
                                        >
                                            "Cloud & Infra"
                                        </button>
                                    </div>
                                </div>

                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                                    {move || filtered_contributions().into_iter().map(|item| {
                                        view! {
                                            <a
                                                href=item.url
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="p-3.5 rounded-2xl bg-neutral-900/40 hover:bg-neutral-900/70 border border-white/10 hover:border-orange-500/40 transition-all flex flex-col justify-between gap-2 group"
                                            >
                                                <div class="flex items-center justify-between">
                                                    <span class="font-bold text-white text-sm group-hover:text-orange-400 transition-colors">
                                                        {item.org}
                                                    </span>
                                                    <span class="text-xs font-mono text-orange-400 bg-orange-500/10 border border-orange-500/20 px-2 py-0.5 rounded-full">
                                                        {item.pr}
                                                    </span>
                                                </div>
                                                <div class="font-mono text-xs text-neutral-300">
                                                    {item.repo}
                                                </div>
                                                <div class="text-[11px] text-neutral-400 leading-snug">
                                                    {item.highlight}
                                                </div>
                                                <div class="text-[10px] text-orange-400/80 font-mono group-hover:translate-x-1 transition-transform inline-flex items-center gap-1 pt-1">
                                                    "View on GitHub →"
                                                </div>
                                            </a>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>

                            // Work Experience
                            <section class="space-y-4">
                                <div class="flex items-center gap-2 pb-2 border-b border-white/10">
                                    <span class="text-sm font-bold uppercase tracking-wider text-orange-400 font-mono">"// Career Timeline"</span>
                                    <h2 class="text-lg font-bold text-white">"Professional Experience"</h2>
                                </div>
                                <div class="space-y-4">
                                    {EXPERIENCES.iter().map(|exp| {
                                        view! {
                                            <div class="p-5 rounded-2xl bg-neutral-900/40 border border-white/10 space-y-3">
                                                <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-1">
                                                    <div>
                                                        <h3 class="font-bold text-white text-base">{exp.role}</h3>
                                                        <div class="text-sm font-semibold text-orange-400">{exp.company}</div>
                                                    </div>
                                                    <div class="text-xs font-mono text-neutral-400 sm:text-right">
                                                        <div>{exp.period}</div>
                                                        <div>{exp.location}</div>
                                                    </div>
                                                </div>
                                                <ul class="space-y-1.5 text-xs sm:text-sm text-neutral-300 list-disc list-inside">
                                                    {exp.bullets.iter().map(|b| {
                                                        view! { <li>{*b}</li> }
                                                    }).collect::<Vec<_>>()}
                                                </ul>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>

                            // Featured Projects
                            <section class="space-y-4">
                                <div class="flex items-center gap-2 pb-2 border-b border-white/10">
                                    <span class="text-sm font-bold uppercase tracking-wider text-orange-400 font-mono">"// Flagship Projects"</span>
                                    <h2 class="text-lg font-bold text-white">"High-Impact Engineering"</h2>
                                </div>
                                <div class="grid grid-cols-1 gap-4">
                                    {PROJECTS.iter().map(|proj| {
                                        view! {
                                            <div class="p-5 rounded-2xl bg-neutral-900/40 border border-white/10 space-y-3">
                                                <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
                                                    <h3 class="font-bold text-white text-base">{proj.title}</h3>
                                                    <span class="inline-block text-xs font-mono font-bold px-2.5 py-1 rounded-full bg-orange-500/20 text-orange-300 border border-orange-500/40 w-fit">
                                                        {proj.badge}
                                                    </span>
                                                </div>
                                                <p class="text-xs sm:text-sm text-neutral-300">{proj.description}</p>
                                                <div class="flex flex-wrap gap-1.5 pt-1">
                                                    {proj.tags.iter().map(|tag| {
                                                        view! {
                                                            <span class="text-[10px] font-mono px-2 py-0.5 rounded-md bg-white/5 border border-white/10 text-neutral-400">
                                                                {*tag}
                                                            </span>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                                <ul class="space-y-1 text-xs text-neutral-400 list-disc list-inside pt-1">
                                                    {proj.bullets.iter().map(|b| {
                                                        view! { <li>{*b}</li> }
                                                    }).collect::<Vec<_>>()}
                                                </ul>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>

                            // Achievements & Honors
                            <section class="space-y-4">
                                <div class="flex items-center gap-2 pb-2 border-b border-white/10">
                                    <span class="text-sm font-bold uppercase tracking-wider text-orange-400 font-mono">"// Recognition"</span>
                                    <h2 class="text-lg font-bold text-white">"Awards & Achievements"</h2>
                                </div>
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                                    {ACHIEVEMENTS.iter().map(|ach| {
                                        view! {
                                            <div class="p-4 rounded-2xl bg-neutral-900/40 border border-white/10 flex items-start gap-3">
                                                <span class="text-2xl">{ach.icon}</span>
                                                <div>
                                                    <div class="font-bold text-sm text-white">{ach.title}</div>
                                                    <div class="text-xs text-neutral-400 mt-0.5">{ach.subtitle}</div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>

                            // Certifications
                            <section class="space-y-4">
                                <div class="flex items-center gap-2 pb-2 border-b border-white/10">
                                    <span class="text-sm font-bold uppercase tracking-wider text-orange-400 font-mono">"// Verification"</span>
                                    <h2 class="text-lg font-bold text-white">"Professional Certifications"</h2>
                                </div>
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
                                    {CERTIFICATIONS.iter().map(|cert| {
                                        view! {
                                            <div class="p-4 rounded-2xl bg-neutral-900/40 border border-white/10 flex items-center gap-3">
                                                <span class="text-2xl">{cert.icon}</span>
                                                <div>
                                                    <div class="font-bold text-xs text-white">{cert.title}</div>
                                                    <div class="text-[11px] font-mono text-orange-400 mt-0.5">{cert.issuer}</div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </section>
                        </div>
                    }.into_any(),
                }}

                // Footer
                <footer class="pt-8 pb-16 text-center text-xs text-neutral-500 space-y-2 font-mono border-t border-white/10">
                    <div class="text-neutral-400 tracking-wider">
                        "BETTER TECHNOLOGY • HAPPIER PEOPLE"
                    </div>
                    <div>
                        "Powered by Leptos 0.7 & Rust WebAssembly • Built by Reginald Alfret V • "
                        <a href="/Reginald_Alfret_Resume.pdf" class="text-orange-400 hover:underline">
                            "Download Official PDF"
                        </a>
                    </div>
                </footer>
            </main>

            // Floating Bottom Stack Dock (All 10 Stacks)
            <div class="floating-stack-dock fixed bottom-4 left-1/2 -translate-x-1/2 z-50 no-print max-w-[95vw] overflow-x-auto">
                <div class="backdrop-blur-xl bg-black/90 border border-white/15 rounded-full py-1.5 px-3.5 shadow-2xl flex items-center gap-1.5 text-xs whitespace-nowrap">
                    <span class="text-[11px] font-mono text-neutral-400 hidden sm:inline mr-1">"Stack:"</span>
                    
                    <a href="/" target="_top" class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition-colors font-medium" title="Switch to Next.js (Production)">
                        <span>"Next.js"</span>
                    </a>
                    <a href="/stacks/nuxt" target="_top" class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition-colors font-medium" title="Switch to Nuxt 3 (Vue 3)">
                        <span>"Nuxt 3"</span>
                    </a>
                    <a href="/stacks/django" target="_top" class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition-colors font-medium" title="Switch to Django (Python 3.13)">
                        <span>"Django"</span>
                    </a>
                    <a href="/stacks/springboot" target="_top" class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition-colors font-medium" title="Switch to Spring Boot (Java 21)">
                        <span>"Spring Boot"</span>
                    </a>
                    <a href="/stacks/laravel" target="_top" class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition-colors font-medium" title="Switch to Laravel (PHP 8.4)">
                        <span>"Laravel"</span>
                    </a>
                    <a href="/stacks/rails" target="_top" class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition-colors font-medium" title="Switch to Rails 8 (Ruby 3.4)">
                        <span>"Rails"</span>
                    </a>
                    <a href="/stacks/angular" target="_top" class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition-colors font-medium" title="Switch to Angular (TypeScript 6.0)">
                        <span>"Angular"</span>
                    </a>
                    <a href="/stacks/blazor" target="_top" class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition-colors font-medium" title="Switch to Blazor (.NET 8 WASM)">
                        <span>"Blazor"</span>
                    </a>
                    <a href="/stacks/buffalo" target="_top" class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition-colors font-medium" title="Switch to Go Buffalo (Go 1.27)">
                        <span>"Buffalo"</span>
                    </a>
                    
                    <span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-orange-500/20 text-orange-300 border border-orange-500/40 font-semibold shadow-sm">
                        <span class="w-1.5 h-1.5 rounded-full bg-orange-400 animate-pulse"></span>
                        "Leptos"
                    </span>
                    <a href="/stacks/jaspr" target="_top" class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition-colors font-medium" title="Switch to Jaspr (Dart SSR & Hydration)">
                        <span>"Jaspr"</span>
                    </a>
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
