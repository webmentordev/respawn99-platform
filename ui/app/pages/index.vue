<script setup>
useHead({
    title: 'respawn99 - Leveled game accounts by a team of friends',
    meta: [
        { name: 'description', content: 'A team of friends selling leveled Rust, Call of Duty MW, War Dogs and Fortnite accounts, plus other games on request. Full access and easy credential changes.' }
    ]
})

const games = [
    {
        id: 'rust',
        name: 'Rust',
        tag: 'Leveled by our team, skins and blueprints earned in raids',
        count: 9,
        from: 14,
        tint: 'from-ember/30 via-rust-deep/20 to-transparent',
        ring: 'hover:border-ember/60',
        dot: 'bg-ember'
    },
    {
        id: 'mw',
        name: 'Call of Duty MW',
        tag: 'Levels, camos and prestige unlocked by our squad',
        count: 14,
        from: 19,
        tint: 'from-mw/25 via-mw-deep/20 to-transparent',
        ring: 'hover:border-mw/60',
        dot: 'bg-mw'
    },
    {
        id: 'wardogs',
        name: 'War Dogs',
        tag: 'Ranked up together with gear and currency',
        count: 6,
        from: 9,
        tint: 'from-wardogs/25 via-wardogs-deep/20 to-transparent',
        ring: 'hover:border-wardogs/60',
        dot: 'bg-wardogs'
    },
    {
        id: 'fortnite',
        name: 'Fortnite',
        tag: 'High level accounts with skins, emotes and battle passes',
        count: 8,
        from: 12,
        tint: 'from-violet-500/25 via-violet-900/20 to-transparent',
        ring: 'hover:border-violet-500/60',
        dot: 'bg-violet-500'
    }
]

const otherGames = [
    'Apex Legends',
    'Valorant',
    'Counter-Strike 2',
    'GTA V Online',
    'Warzone',
    'Rainbow Six Siege',
    'Minecraft',
    'Roblox',
    'League of Legends',
    'Overwatch 2'
]

const { accounts } = useAccounts()

const platforms = [
    { name: 'Steam', icon: 'simple-icons:steam' },
    { name: 'Activision', icon: 'simple-icons:activision' },
    { name: 'Battle.net', icon: 'simple-icons:battledotnet' },
    { name: 'Xbox', icon: 'simple-icons:xbox' },
    { name: 'PlayStation', icon: 'simple-icons:playstation' },
    { name: 'Epic Games', icon: 'simple-icons:epicgames' },
    { name: 'EA', icon: 'simple-icons:ea' },
    { name: 'Ubisoft', icon: 'simple-icons:ubisoft' },
    { name: 'Riot Games', icon: 'simple-icons:riotgames' },
    { name: 'GOG', icon: 'simple-icons:gogdotcom' },
    { name: 'Nintendo', icon: 'simple-icons:nintendoswitch' },
    { name: 'Rockstar', icon: 'simple-icons:rockstargames' }
]

const active = ref('all')

const filtered = computed(() =>
    active.value === 'all' ? accounts : accounts.filter((a) => a.game === active.value)
)

const gameMeta = (id) => games.find((g) => g.id === id)

const tabs = [
    { id: 'all', label: 'All games' },
    { id: 'rust', label: 'Rust' },
    { id: 'mw', label: 'Call of Duty MW' },
    { id: 'wardogs', label: 'War Dogs' },
    { id: 'fortnite', label: 'Fortnite' }
]

const stats = [
    { value: 'Team', label: 'Friends who play every day' },
    { value: 'Full', label: 'Access to every login' },
    { value: '4.9/5', label: 'From past buyers' }
]

const weeks = [14, 22, 18, 30, 26, 34, 28, 38, 32, 41, 36, 44]

const playtimeLog = [
    { title: 'Hours tracked per game', body: 'Playtime comes straight from Steam, Battle.net and Activision, so the number you see is the number that was played.' },
    { title: 'Progress proof before you pay', body: 'Ask and we will send screenshots of the stats page, unlock lists and play history for the exact account.' },
    { title: 'Leveled by our team', body: 'A group of friends plays together and levels each account in normal matches. No bots and no cheat software.' }
]

const access = [
    { label: 'Email', note: 'Swap to your own address' },
    { label: 'Password', note: 'Set a new one right away' },
    { label: 'Phone number', note: 'Remove or replace' },
    { label: 'Two-factor', note: 'Enable on your device' },
    { label: 'Linked accounts', note: 'Re-link to yours' },
    { label: 'Security questions', note: 'Reset to yours' }
]

const steps = [
    { title: 'Choose an account', body: 'Pick a listing and message us. We will confirm it is still available and send the playtime proof.' },
    { title: 'Pay', body: 'Card, crypto or wallet. Simple checkout, no sign-up needed.' },
    { title: 'Get the original logins', body: 'You receive the account email, password and recovery details directly from us.' },
    { title: 'Change everything', body: 'Update email and credentials with the short guide we include. After that, only you can get in.' }
]

const faqs = [
    {
        q: 'Is this a marketplace with many sellers?',
        a: 'No. respawn99 is a team of friends who play these games together. Every account here was leveled by us and we handle every order ourselves.'
    },
    {
        q: 'Do you sell other games?',
        a: 'Yes. Besides Rust, Call of Duty MW, War Dogs and Fortnite, we level accounts for Apex Legends, Valorant, CS2, GTA V and more. Message us with the game you want.'
    },
    {
        q: 'How do I know the accounts were played legit?',
        a: 'Each account has tracked playtime, level and progress. We share screenshots of play history and unlock stats before you buy.'
    },
    {
        q: 'Do I really get full access?',
        a: 'Yes. You get the original email, password and recovery details, plus a step-by-step guide to change them on each platform.'
    },
    {
        q: 'How fast is delivery?',
        a: 'Usually within minutes of payment. If we are offline, it is delivered as soon as we are back, never later than a few hours.'
    },
    {
        q: 'What if something goes wrong after I buy?',
        a: 'Message us. If the account has an issue that existed before the sale, we will fix it or replace it.'
    },
    {
        q: 'What payments do you accept?',
        a: 'Visa, Mastercard, Apple Pay, Google Pay, USDT, Bitcoin and Litecoin.'
    }
]

const openFaq = ref(0)
</script>

<template>
    <div class="min-h-screen selection:bg-ember/40 selection:text-white">
        <section class="relative overflow-hidden">
            <div class="pointer-events-none absolute inset-0">
                <div
                    class="absolute -top-40 left-1/2 h-[560px] w-[900px] -translate-x-1/2 rounded-full bg-ember/10 blur-[140px]" />
                <div class="absolute right-[-10%] top-40 h-[420px] w-[420px] rounded-full bg-glow/10 blur-[120px]" />
                <div class="absolute inset-0 bg-grid" />
            </div>

            <Navbar />

            <div
                class="relative mx-auto grid max-w-7xl items-center gap-14 px-5 pb-24 pt-20 lg:grid-cols-[1.1fr_0.9fr] lg:pt-28">
                <div>
                    <div
                        class="mb-7 inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 py-1.5 pl-2 pr-4 text-sm text-zinc-300">
                        <span class="relative flex h-2.5 w-2.5">
                            <span
                                class="absolute inline-flex h-full w-full animate-ping rounded-full bg-emerald-400 opacity-60" />
                            <span class="relative inline-flex h-2.5 w-2.5 rounded-full bg-emerald-400" />
                        </span>
                        <span>A team of friends, not a marketplace</span>
                    </div>

                    <h1
                        class="text-5xl font-bold leading-[1.02] tracking-tight text-white sm:text-6xl lg:text-7xl font-display">
                        Accounts our team leveled.<br />
                        <span
                            class="bg-gradient-to-r from-ember-light via-flare to-hot bg-clip-text text-transparent">Yours
                            to keep.</span>
                    </h1>

                    <p class="mt-7 max-w-xl text-lg leading-relaxed text-zinc-400">
                        We are a team of friends who play Rust, Call of Duty MW, War Dogs, Fortnite and more every day.
                        We take each account to a high level together, track the playtime, and hand you full access to
                        change the email and every credential in minutes.
                    </p>

                    <div class="mt-10 flex flex-col gap-3 sm:flex-row">
                        <a href="#accounts"
                            class="inline-flex items-center justify-center gap-2 rounded-xl bg-brand px-7 py-4 text-base font-semibold text-white shadow-lg shadow-hot/20 transition hover:brightness-110">
                            Browse accounts
                            <Icon name="lucide:arrow-right" class="h-4 w-4" />
                        </a>
                        <a href="#contact"
                            class="inline-flex items-center justify-center gap-2 rounded-xl border border-white/10 bg-white/5 px-7 py-4 text-base font-semibold text-white transition hover:bg-white/10">
                            <Icon name="simple-icons:discord" class="h-4 w-4" />
                            Message us directly
                        </a>
                    </div>

                    <dl class="mt-14 grid max-w-xl grid-cols-3 gap-6 border-t border-white/10 pt-8">
                        <div v-for="s in stats" :key="s.label">
                            <dt class="text-2xl font-bold text-white sm:text-3xl font-display">{{ s.value }}</dt>
                            <dd class="mt-1 text-sm text-zinc-500">{{ s.label }}</dd>
                        </div>
                    </dl>
                </div>

                <div class="relative mx-auto w-full max-w-md lg:max-w-none">
                    <div
                        class="absolute -inset-6 rounded-[2rem] bg-gradient-to-tr from-ember/20 via-transparent to-glow/20 blur-2xl" />

                    <div class="relative rounded-3xl border border-white/10 bg-panel/90 p-5 shadow-2xl backdrop-blur">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center gap-3">
                                <div
                                    class="grid h-12 w-12 place-items-center rounded-xl bg-gradient-to-br from-ember to-rust-deep text-white">
                                    <Icon name="simple-icons:steam" class="h-6 w-6" />
                                </div>
                                <div>
                                    <p class="font-semibold text-white">Full blueprint vault</p>
                                    <p class="text-sm text-zinc-500">Rust · Steam</p>
                                </div>
                            </div>
                            <span
                                class="inline-flex items-center gap-1.5 rounded-full bg-emerald-400/10 px-3 py-1 text-xs font-semibold text-emerald-400">
                                <Icon name="lucide:badge-check" class="h-3.5 w-3.5" />
                                Team leveled
                            </span>
                        </div>

                        <div class="mt-5 rounded-2xl border border-white/5 bg-white/[0.03] p-4">
                            <div class="flex items-end justify-between">
                                <div>
                                    <p class="text-xs text-zinc-500">Total playtime</p>
                                    <p class="text-3xl font-bold text-white font-display">4,200 hrs</p>
                                </div>
                                <p class="text-xs text-zinc-500">Last 12 weeks</p>
                            </div>
                            <div class="mt-4 flex h-20 items-end gap-1.5">
                                <div v-for="(w, i) in weeks" :key="i"
                                    class="flex-1 rounded-t bg-gradient-to-t from-ember/40 to-hot"
                                    :style="{ height: (w / 44) * 100 + '%' }" />
                            </div>
                        </div>

                        <div class="mt-4 grid grid-cols-3 gap-3">
                            <div v-for="s in [{ v: '62', l: 'Skins' }, { v: '210', l: 'Level' }, { v: '0', l: 'Bans' }]"
                                :key="s.l" class="rounded-xl border border-white/5 bg-white/[0.03] p-3 text-center">
                                <p class="text-xl font-bold text-white font-display">{{ s.v }}</p>
                                <p class="text-xs text-zinc-500">{{ s.l }}</p>
                            </div>
                        </div>

                        <div class="mt-5 flex items-center justify-between border-t border-white/5 pt-5">
                            <div>
                                <p class="text-xs text-zinc-500">Price</p>
                                <p class="text-3xl font-bold text-white font-display">$89</p>
                            </div>
                            <a href="#contact"
                                class="rounded-xl bg-white px-6 py-3 text-sm font-semibold text-black transition hover:bg-zinc-200">Buy
                                now</a>
                        </div>
                    </div>

                    <div
                        class="absolute -bottom-6 -left-4 hidden items-center gap-3 rounded-2xl border border-white/10 bg-float/95 px-4 py-3 shadow-xl backdrop-blur sm:flex">
                        <span class="grid h-9 w-9 place-items-center rounded-full bg-emerald-400/15 text-emerald-400">
                            <Icon name="lucide:key-round" class="h-4 w-4" />
                        </span>
                        <div>
                            <p class="text-sm font-semibold text-white">Full access handed over</p>
                            <p class="text-xs text-zinc-500">Email and password changeable</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="border-y border-white/5 bg-white/[0.015]">
            <div class="mx-auto max-w-7xl px-5 py-10">
                <p class="text-center text-sm text-zinc-500">Accounts available on every major platform</p>
                <div class="mt-7 grid grid-cols-3 gap-x-6 gap-y-6 sm:grid-cols-4 md:grid-cols-6">
                    <div v-for="p in platforms" :key="p.name" class="group flex flex-col items-center gap-2.5">
                        <span
                            class="grid h-14 w-14 place-items-center rounded-2xl border border-white/10 bg-white/5 text-zinc-400 transition group-hover:border-white/25 group-hover:text-white">
                            <Icon :name="p.icon" class="h-7 w-7" />
                        </span>
                        <span class="text-xs font-medium text-zinc-500 transition group-hover:text-zinc-300">{{ p.name
                        }}</span>
                    </div>
                </div>
            </div>
        </section>

        <section id="games" class="mx-auto max-w-7xl px-5 py-20">
            <div class="mb-12 max-w-2xl">
                <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl font-display">Pick your game</h2>
                <p class="mt-4 text-lg text-zinc-400">A small, carefully kept stock. Every account here was leveled by
                    our team through real matches, not shortcuts.</p>
            </div>

            <div class="grid gap-5 sm:grid-cols-2 lg:grid-cols-4">
                <button v-for="g in games" :key="g.id"
                    class="group relative overflow-hidden rounded-3xl border border-white/10 bg-surface p-7 text-left transition"
                    :class="g.ring"
                    @click="active = g.id; $nextTick(() => document.getElementById('accounts')?.scrollIntoView({ behavior: 'smooth' }))">
                    <div class="absolute inset-0 bg-gradient-to-br opacity-70 transition group-hover:opacity-100"
                        :class="g.tint" />
                    <div class="relative flex h-56 flex-col justify-between">
                        <div class="flex items-center justify-between">
                            <span
                                class="flex items-center gap-2 rounded-full bg-black/40 px-3 py-1 text-xs font-medium text-zinc-300">
                                <span class="h-2 w-2 rounded-full" :class="g.dot" />
                                {{ g.count }} in stock
                            </span>
                            <span
                                class="grid h-10 w-10 place-items-center rounded-full border border-white/15 text-white transition group-hover:bg-white group-hover:text-black">
                                <Icon name="lucide:arrow-up-right" class="h-4 w-4" />
                            </span>
                        </div>
                        <div>
                            <h3 class="text-2xl font-bold text-white font-display">{{ g.name }}</h3>
                            <p class="mt-2 text-sm text-zinc-400">{{ g.tag }}</p>
                            <p class="mt-4 text-sm text-zinc-300">From <span class="font-bold text-white">${{ g.from
                            }}</span></p>
                        </div>
                    </div>
                </button>
            </div>

            <div class="mt-8 rounded-3xl border border-white/10 bg-white/[0.03] p-6 sm:p-8">
                <h3 class="text-xl font-semibold text-white font-display">Other games on request</h3>
                <p class="mt-2 text-zinc-400">Do not see your game? We level accounts for more titles. Message us and
                    we will tell you what we can deliver.</p>
                <div class="mt-5 flex flex-wrap gap-2.5">
                    <span v-for="o in otherGames" :key="o"
                        class="rounded-full border border-white/10 bg-white/5 px-4 py-1.5 text-sm text-zinc-300">{{ o
                        }}</span>
                    <a href="#contact"
                        class="rounded-full bg-white px-4 py-1.5 text-sm font-semibold text-black transition hover:bg-zinc-200">And
                        more</a>
                </div>
            </div>
        </section>

        <section id="accounts" class="mx-auto max-w-7xl px-5 py-20">
            <div class="mb-10 flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
                <div class="max-w-xl">
                    <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl font-display">Accounts in stock
                    </h2>
                    <p class="mt-4 text-lg text-zinc-400">Each listing shows its tracked playtime and exactly what is
                        unlocked.</p>
                </div>

                <div class="flex flex-wrap gap-2 rounded-2xl border border-white/10 bg-white/[0.03] p-1.5">
                    <button v-for="t in tabs" :key="t.id" class="rounded-xl px-4 py-2 text-sm font-semibold transition"
                        :class="active === t.id ? 'bg-white text-black' : 'text-zinc-400 hover:text-white'"
                        @click="active = t.id">
                        {{ t.label }}
                    </button>
                </div>
            </div>

            <div class="grid gap-5 sm:grid-cols-2 lg:grid-cols-3">
                <article v-for="a in filtered" :key="a.id"
                    class="group relative flex flex-col overflow-hidden rounded-3xl border border-white/10 bg-surface transition hover:-translate-y-1 hover:border-white/20">
                    <div class="relative h-36 overflow-hidden bg-gradient-to-br" :class="gameMeta(a.game).tint">
                        <div
                            class="absolute inset-0 bg-[radial-gradient(circle_at_80%_0%,rgba(255,255,255,0.08),transparent_55%)]" />
                        <div
                            class="absolute left-5 top-5 flex items-center gap-2 rounded-full bg-black/40 px-3 py-1 text-xs font-semibold text-white backdrop-blur">
                            <span class="h-2 w-2 rounded-full" :class="gameMeta(a.game).dot" />
                            {{ gameMeta(a.game).name }}
                        </div>
                        <div
                            class="absolute right-5 top-5 flex items-center gap-1.5 rounded-full bg-black/40 px-3 py-1 text-xs font-semibold text-white backdrop-blur">
                            <Icon :name="a.platform" class="h-3.5 w-3.5" />
                            {{ a.platformName }}
                        </div>
                        <div class="absolute bottom-4 left-5">
                            <p class="text-xs text-zinc-300/80">Tracked playtime</p>
                            <p class="text-xl font-bold text-white font-display">{{ a.hours.toLocaleString() }} hrs</p>
                        </div>
                    </div>

                    <div class="flex flex-1 flex-col p-6">
                        <NuxtLink :to="`/accounts/${a.id}`"
                            class="text-xl font-semibold text-white after:absolute after:inset-0 hover:underline">{{
                                a.title }}</NuxtLink>
                        <ul class="mt-4 space-y-2 text-sm text-zinc-400">
                            <li v-for="i in a.items" :key="i" class="flex items-center gap-2.5">
                                <Icon name="lucide:check" class="h-4 w-4 shrink-0 text-emerald-400" />
                                {{ i }}
                            </li>
                            <li class="flex items-center gap-2.5">
                                <Icon name="lucide:check" class="h-4 w-4 shrink-0 text-emerald-400" />
                                Full access, change email and password
                            </li>
                        </ul>

                        <div class="mt-auto flex items-end justify-between pt-7">
                            <div>
                                <p class="text-3xl font-bold text-white font-display">${{ a.price }}</p>
                                <p class="mt-0.5 text-xs" :class="a.stock <= 1 ? 'text-amber-400' : 'text-zinc-500'">{{
                                    a.stock === 1 ? 'Only 1 available' : a.stock + ' available' }}</p>
                            </div>
                            <span
                                class="rounded-xl bg-white/10 px-5 py-2.5 text-sm font-semibold text-white transition group-hover:bg-white group-hover:text-black">View
                                details</span>
                        </div>
                    </div>
                </article>
            </div>
        </section>

        <section id="playtime" class="mx-auto max-w-7xl px-5 py-20">
            <div class="grid items-center gap-14 lg:grid-cols-2">
                <div>
                    <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl font-display">Leveled by our
                        team, tracked and provable</h2>
                    <p class="mt-4 max-w-lg text-lg text-zinc-400">These are not farmed or flipped accounts. We played
                        them together like any other, and we keep the records to show it.</p>

                    <div class="mt-10 space-y-7">
                        <div v-for="p in playtimeLog" :key="p.title" class="flex gap-4">
                            <span
                                class="mt-1 grid h-9 w-9 shrink-0 place-items-center rounded-lg bg-emerald-400/10 text-emerald-400">
                                <Icon name="lucide:check" class="h-5 w-5" />
                            </span>
                            <div>
                                <h3 class="font-semibold text-white">{{ p.title }}</h3>
                                <p class="mt-1 leading-relaxed text-zinc-400">{{ p.body }}</p>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="rounded-3xl border border-white/10 bg-surface p-6 sm:p-8">
                    <div class="flex items-center justify-between">
                        <p class="font-semibold text-white">Playtime log</p>
                        <span class="rounded-full bg-white/5 px-3 py-1 text-xs text-zinc-400">Sample account</span>
                    </div>

                    <div class="mt-6 space-y-5">
                        <div v-for="r in [
                            { n: 'Rust', icon: 'simple-icons:steam', h: 4200, p: 100, c: 'from-ember-light to-hot' },
                            { n: 'Call of Duty MW', icon: 'simple-icons:battledotnet', h: 1150, p: 62, c: 'from-mw to-[#2a9d8f]' },
                            { n: 'Fortnite', icon: 'simple-icons:epicgames', h: 760, p: 44, c: 'from-violet-500 to-fuchsia-500' },
                            { n: 'War Dogs', icon: 'simple-icons:steam', h: 380, p: 28, c: 'from-wardogs to-[#e08a1e]' }
                        ]" :key="r.n">
                            <div class="mb-2 flex items-center justify-between text-sm">
                                <span class="flex items-center gap-2 text-zinc-300">
                                    <Icon :name="r.icon" class="h-4 w-4 text-zinc-500" />
                                    {{ r.n }}
                                </span>
                                <span class="font-semibold text-white">{{ r.h.toLocaleString() }} hrs</span>
                            </div>
                            <div class="h-2 overflow-hidden rounded-full bg-white/5">
                                <div class="h-full rounded-full bg-gradient-to-r" :class="r.c"
                                    :style="{ width: r.p + '%' }" />
                            </div>
                        </div>
                    </div>

                    <div class="mt-8 grid grid-cols-2 gap-3">
                        <div class="rounded-2xl border border-white/5 bg-white/[0.03] p-4">
                            <p class="text-xs text-zinc-500">Avg session</p>
                            <p class="mt-1 text-xl font-bold text-white font-display">3.4 hrs</p>
                        </div>
                        <div class="rounded-2xl border border-white/5 bg-white/[0.03] p-4">
                            <p class="text-xs text-zinc-500">Account age</p>
                            <p class="mt-1 text-xl font-bold text-white font-display">3 years</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section id="access" class="mx-auto max-w-7xl px-5 py-20">
            <div class="rounded-[2rem] border border-white/10 bg-gradient-to-b from-raised to-deep p-8 sm:p-14">
                <div class="grid gap-12 lg:grid-cols-[0.9fr_1.1fr]">
                    <div>
                        <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl font-display">Full access,
                            changed in minutes</h2>
                        <p class="mt-4 max-w-md text-lg text-zinc-400">You get the original login details and a short
                            guide for each platform. Replace everything and the account belongs to you alone.</p>

                        <div class="mt-8 flex flex-wrap gap-3">
                            <span v-for="p in platforms.slice(0, 5)" :key="p.name"
                                class="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3.5 py-1.5 text-sm text-zinc-300">
                                <Icon :name="p.icon" class="h-4 w-4" />
                                {{ p.name }}
                            </span>
                        </div>
                    </div>

                    <div class="grid gap-3 sm:grid-cols-2">
                        <div v-for="a in access" :key="a.label"
                            class="flex items-center gap-4 rounded-2xl border border-white/10 bg-white/[0.03] p-4">
                            <span
                                class="grid h-10 w-10 shrink-0 place-items-center rounded-xl bg-gradient-to-br from-ember/20 to-hot/20 text-ember-light">
                                <Icon name="lucide:lock-keyhole-open" class="h-5 w-5" />
                            </span>
                            <div>
                                <p class="font-semibold text-white">{{ a.label }}</p>
                                <p class="text-sm text-zinc-500">{{ a.note }}</p>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="relative mt-16 grid gap-10 md:grid-cols-4">
                    <div
                        class="absolute left-0 right-0 top-6 hidden h-px bg-gradient-to-r from-transparent via-white/15 to-transparent md:block" />
                    <div v-for="(s, i) in steps" :key="s.title" class="relative">
                        <span
                            class="relative z-10 grid h-12 w-12 place-items-center rounded-full border border-white/15 bg-deep text-lg font-bold text-white font-display">{{
                                i + 1 }}</span>
                        <h3 class="mt-6 text-lg font-semibold text-white">{{ s.title }}</h3>
                        <p class="mt-2 max-w-xs leading-relaxed text-zinc-400">{{ s.body }}</p>
                    </div>
                </div>
            </div>
        </section>

        <section id="faq" class="mx-auto max-w-3xl px-5 py-20">
            <h2 class="text-center text-3xl font-bold tracking-tight text-white sm:text-4xl font-display">Questions
                before you buy</h2>

            <div class="mt-12 divide-y divide-white/10 rounded-3xl border border-white/10 bg-surface">
                <div v-for="(f, i) in faqs" :key="f.q">
                    <button class="flex w-full items-center justify-between gap-6 px-6 py-5 text-left"
                        :aria-expanded="openFaq === i" @click="openFaq = openFaq === i ? null : i">
                        <span class="font-semibold text-white">{{ f.q }}</span>
                        <Icon name="lucide:plus"
                            class="h-5 w-5 shrink-0 text-zinc-400 transition-transform duration-300"
                            :class="openFaq === i ? 'rotate-45' : ''" />
                    </button>
                    <div class="grid transition-all duration-300"
                        :class="openFaq === i ? 'grid-rows-[1fr]' : 'grid-rows-[0fr]'">
                        <div class="overflow-hidden">
                            <p class="px-6 pb-6 leading-relaxed text-zinc-400">{{ f.a }}</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section id="contact" class="mx-auto max-w-7xl px-5 pb-24 pt-10">
            <div
                class="relative overflow-hidden rounded-[2rem] border border-white/10 bg-panel px-8 py-16 text-center sm:px-16 sm:py-20">
                <div
                    class="pointer-events-none absolute left-1/2 top-0 h-72 w-[640px] -translate-x-1/2 rounded-full bg-ember/20 blur-[110px]" />
                <div class="relative">
                    <h2 class="mx-auto max-w-2xl text-4xl font-bold tracking-tight text-white sm:text-5xl font-display">
                        Talk to us, not a support queue</h2>
                    <p class="mx-auto mt-5 max-w-lg text-lg text-zinc-400">Tell us which account or game you want and
                        we will reply personally with proof of playtime and payment options.</p>

                    <div class="mt-9 flex flex-col items-center justify-center gap-3 sm:flex-row">
                        <a href="#"
                            class="inline-flex items-center gap-2.5 rounded-xl bg-discord px-7 py-4 text-base font-semibold text-white transition hover:brightness-110">
                            <Icon name="simple-icons:discord" class="h-5 w-5" />
                            Discord
                        </a>
                        <a href="#"
                            class="inline-flex items-center gap-2.5 rounded-xl bg-telegram px-7 py-4 text-base font-semibold text-white transition hover:brightness-110">
                            <Icon name="simple-icons:telegram" class="h-5 w-5" />
                            Telegram
                        </a>
                        <a href="#"
                            class="inline-flex items-center gap-2.5 rounded-xl border border-white/10 bg-white/5 px-7 py-4 text-base font-semibold text-white transition hover:bg-white/10">
                            <Icon name="lucide:mail" class="h-5 w-5" />
                            Email
                        </a>
                    </div>
                </div>
            </div>
        </section>

        <Footer />
    </div>
</template>