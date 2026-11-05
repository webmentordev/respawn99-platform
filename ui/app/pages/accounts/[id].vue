<script setup>
const route = useRoute()
const { accounts, getAccount, getGame } = useAccounts()

const account = computed(() => getAccount(route.params.id))

if (!account.value) {
    throw createError({ statusCode: 404, statusMessage: 'Account not found', fatal: true })
}

const game = computed(() => getGame(account.value.game))

const related = computed(() =>
    accounts.filter((a) => a.id !== account.value.id).sort((a, b) => Number(b.game === account.value.game) - Number(a.game === account.value.game)).slice(0, 3)
)

const activeShot = ref(0)
const copied = ref(false)
const menu = ref(false)

const maxWeek = computed(() => Math.max(...account.value.weeks))

const copyCode = async () => {
    try {
        await navigator.clipboard.writeText(account.value.code)
        copied.value = true
        setTimeout(() => (copied.value = false), 1800)
    } catch {
        copied.value = false
    }
}

const handover = [
    { icon: 'lucide:mail', title: 'Original email access', body: 'Full login to the email linked to the account.' },
    { icon: 'lucide:key-round', title: 'Account password', body: 'Current password and recovery codes.' },
    { icon: 'lucide:file-text', title: 'Change guide', body: 'Step-by-step guide to replace every credential.' }
]

const guarantees = [
    { icon: 'lucide:badge-check', text: 'Played legit, playtime tracked' },
    { icon: 'lucide:lock-keyhole-open', text: 'Full access, easy credential change' },
    { icon: 'lucide:zap', text: 'Delivered within minutes of payment' },
    { icon: 'lucide:life-buoy', text: 'Fix or replacement for pre-sale issues' }
]

useHead({
    title: `${account.value.title} · ${game.value.name} account - respawn99`,
    meta: [{ name: 'description', content: account.value.summary }],
    link: [
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
        {
            rel: 'stylesheet',
            href: 'https://fonts.googleapis.com/css2?family=Chakra+Petch:wght@500;600;700&family=Manrope:wght@400;500;600;700&display=swap'
        }
    ]
})
</script>

<template>
    <div v-if="account"
        class="min-h-screen bg-[#090a0f] text-zinc-300 antialiased selection:bg-[#ff6a3d]/40 selection:text-white"
        style="font-family: 'Manrope', system-ui, sans-serif">
        <header class="sticky top-0 z-50 border-b border-white/5 bg-[#090a0f]/80 backdrop-blur-xl">
            <div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-5">
                <NuxtLink to="/" class="flex items-center gap-2.5">
                    <span
                        class="grid h-8 w-8 place-items-center rounded-lg bg-gradient-to-br from-[#ff6a3d] to-[#ff2e63] text-white">
                        <Icon name="lucide:crosshair" class="h-4.5 w-4.5" />
                    </span>
                    <span class="text-lg font-semibold tracking-tight text-white"
                        style="font-family: 'Chakra Petch', sans-serif">respawn99</span>
                </NuxtLink>

                <nav class="hidden items-center gap-8 text-sm font-medium md:flex">
                    <NuxtLink to="/#games" class="transition hover:text-white">Games</NuxtLink>
                    <NuxtLink to="/#accounts" class="transition hover:text-white">Accounts</NuxtLink>
                    <NuxtLink to="/#playtime" class="transition hover:text-white">Playtime</NuxtLink>
                    <NuxtLink to="/#faq" class="transition hover:text-white">FAQ</NuxtLink>
                </nav>

                <div class="hidden items-center gap-3 md:flex">
                    <a href="#buy"
                        class="rounded-lg bg-white px-4 py-2 text-sm font-semibold text-black transition hover:bg-zinc-200">Buy
                        this account</a>
                </div>

                <button class="grid h-10 w-10 place-items-center rounded-lg border border-white/10 md:hidden"
                    aria-label="Toggle menu" @click="menu = !menu">
                    <Icon :name="menu ? 'lucide:x' : 'lucide:menu'" class="h-5 w-5" />
                </button>
            </div>

            <div v-if="menu" class="border-t border-white/5 px-5 pb-5 pt-3 md:hidden">
                <div class="flex flex-col gap-1 text-sm font-medium">
                    <NuxtLink to="/#games" class="rounded-lg px-3 py-2.5 hover:bg-white/5">Games</NuxtLink>
                    <NuxtLink to="/#accounts" class="rounded-lg px-3 py-2.5 hover:bg-white/5">Accounts</NuxtLink>
                    <NuxtLink to="/#playtime" class="rounded-lg px-3 py-2.5 hover:bg-white/5">Playtime</NuxtLink>
                    <NuxtLink to="/#faq" class="rounded-lg px-3 py-2.5 hover:bg-white/5">FAQ</NuxtLink>
                    <a href="#buy" class="mt-2 rounded-lg bg-white px-4 py-3 text-center font-semibold text-black"
                        @click="menu = false">Buy this account</a>
                </div>
            </div>
        </header>

        <main class="relative">
            <div class="pointer-events-none absolute inset-x-0 top-0 h-[480px] overflow-hidden">
                <div
                    class="absolute -top-40 left-1/2 h-[480px] w-[900px] -translate-x-1/2 rounded-full bg-[#ff6a3d]/10 blur-[140px]" />
                <div
                    class="absolute inset-0 bg-[linear-gradient(to_right,rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(to_bottom,rgba(255,255,255,0.03)_1px,transparent_1px)] bg-[size:56px_56px] [mask-image:radial-gradient(ellipse_at_top,black,transparent_70%)]" />
            </div>

            <div class="relative mx-auto max-w-7xl px-5 pb-24 pt-10">
                <nav class="flex flex-wrap items-center gap-2 text-sm text-zinc-500" aria-label="Breadcrumb">
                    <NuxtLink to="/" class="transition hover:text-white">Home</NuxtLink>
                    <Icon name="lucide:chevron-right" class="h-3.5 w-3.5" />
                    <NuxtLink to="/#accounts" class="transition hover:text-white">Accounts</NuxtLink>
                    <Icon name="lucide:chevron-right" class="h-3.5 w-3.5" />
                    <span class="text-zinc-300">{{ account.title }}</span>
                </nav>

                <div class="mt-8 grid gap-10 lg:grid-cols-[1fr_380px]">
                    <article class="min-w-0">
                        <div class="flex flex-wrap items-center gap-2.5">
                            <span
                                class="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs font-semibold text-white">
                                <span class="h-2 w-2 rounded-full" :class="game.dot" />
                                {{ game.name }}
                            </span>
                            <span
                                class="inline-flex items-center gap-1.5 rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs font-semibold text-white">
                                <Icon :name="account.platform" class="h-3.5 w-3.5" />
                                {{ account.platformName }}
                            </span>
                            <span
                                class="inline-flex items-center gap-1.5 rounded-full bg-emerald-400/10 px-3 py-1 text-xs font-semibold text-emerald-400">
                                <Icon name="lucide:badge-check" class="h-3.5 w-3.5" />
                                Played legit
                            </span>
                        </div>

                        <h1 class="mt-5 text-4xl font-bold leading-tight tracking-tight text-white sm:text-5xl"
                            style="font-family: 'Chakra Petch', sans-serif">{{ account.title }}</h1>
                        <p class="mt-4 max-w-2xl text-lg leading-relaxed text-zinc-400">{{ account.summary }}</p>

                        <div class="mt-6 flex flex-wrap items-center gap-x-6 gap-y-2 text-sm text-zinc-500">
                            <span class="flex items-center gap-2">
                                <Icon name="lucide:user-round" class="h-4 w-4" />
                                Sold by respawn99
                            </span>
                            <span class="flex items-center gap-2">
                                <Icon name="lucide:calendar" class="h-4 w-4" />
                                Last played {{ account.lastPlayed }}
                            </span>
                            <button class="flex items-center gap-2 transition hover:text-white" @click="copyCode">
                                <Icon :name="copied ? 'lucide:check' : 'lucide:copy'" class="h-4 w-4"
                                    :class="copied ? 'text-emerald-400' : ''" />
                                {{ copied ? 'Copied' : account.code }}
                            </button>
                        </div>

                        <div class="mt-9 overflow-hidden rounded-3xl border border-white/10 bg-[#0e1017]">
                            <div class="relative flex h-72 items-center justify-center overflow-hidden bg-gradient-to-br sm:h-96"
                                :class="game.tint">
                                <div
                                    class="absolute inset-0 bg-[radial-gradient(circle_at_80%_0%,rgba(255,255,255,0.1),transparent_55%)]" />
                                <div class="relative text-center">
                                    <span
                                        class="mx-auto grid h-16 w-16 place-items-center rounded-2xl border border-white/15 bg-black/30 text-white backdrop-blur">
                                        <Icon :name="account.gallery[activeShot].icon" class="h-8 w-8" />
                                    </span>
                                    <p class="mt-4 text-xl font-semibold text-white">{{
                                        account.gallery[activeShot].label }}</p>
                                    <p class="mt-1 text-sm text-zinc-300/80">Proof screenshot, full resolution sent
                                        before payment</p>
                                </div>
                            </div>
                            <div class="grid grid-cols-2 gap-2 border-t border-white/10 p-3 sm:grid-cols-4">
                                <button v-for="(g, i) in account.gallery" :key="g.label"
                                    class="flex items-center gap-2.5 rounded-xl px-3 py-2.5 text-left text-sm font-medium transition"
                                    :class="activeShot === i ? 'bg-white text-black' : 'bg-white/[0.03] text-zinc-400 hover:bg-white/[0.07] hover:text-white'"
                                    @click="activeShot = i">
                                    <Icon :name="g.icon" class="h-4 w-4 shrink-0" />
                                    <span class="truncate">{{ g.label }}</span>
                                </button>
                            </div>
                        </div>

                        <dl class="mt-8 grid grid-cols-2 gap-3 sm:grid-cols-4">
                            <div v-for="s in account.stats" :key="s.label"
                                class="rounded-2xl border border-white/10 bg-[#0e1017] p-5">
                                <dd class="text-2xl font-bold text-white"
                                    style="font-family: 'Chakra Petch', sans-serif">{{ s.value }}</dd>
                                <dt class="mt-1 text-sm text-zinc-500">{{ s.label }}</dt>
                            </div>
                        </dl>

                        <section class="mt-14">
                            <h2 class="text-2xl font-bold text-white" style="font-family: 'Chakra Petch', sans-serif">
                                About this account</h2>
                            <div class="mt-5 max-w-2xl space-y-5 text-[17px] leading-[1.8] text-zinc-400">
                                <p v-for="(p, i) in account.description" :key="i">{{ p }}</p>
                            </div>
                        </section>

                        <section class="mt-14">
                            <h2 class="text-2xl font-bold text-white" style="font-family: 'Chakra Petch', sans-serif">
                                What is included</h2>
                            <ul class="mt-5 grid gap-3 sm:grid-cols-2">
                                <li v-for="i in account.items" :key="i"
                                    class="flex items-center gap-3 rounded-2xl border border-white/10 bg-[#0e1017] p-4 text-white">
                                    <span
                                        class="grid h-8 w-8 shrink-0 place-items-center rounded-lg bg-emerald-400/10 text-emerald-400">
                                        <Icon name="lucide:check" class="h-4 w-4" />
                                    </span>
                                    {{ i }}
                                </li>
                                <li
                                    class="flex items-center gap-3 rounded-2xl border border-white/10 bg-[#0e1017] p-4 text-white">
                                    <span
                                        class="grid h-8 w-8 shrink-0 place-items-center rounded-lg bg-emerald-400/10 text-emerald-400">
                                        <Icon name="lucide:check" class="h-4 w-4" />
                                    </span>
                                    Full access and credential change guide
                                </li>
                            </ul>
                        </section>

                        <section class="mt-14">
                            <div class="flex items-end justify-between">
                                <h2 class="text-2xl font-bold text-white"
                                    style="font-family: 'Chakra Petch', sans-serif">Playtime and progress</h2>
                                <span class="text-sm text-zinc-500">Last 12 weeks</span>
                            </div>

                            <div class="mt-5 rounded-3xl border border-white/10 bg-[#0e1017] p-6">
                                <div class="flex items-end justify-between">
                                    <div>
                                        <p class="text-sm text-zinc-500">Total tracked playtime</p>
                                        <p class="text-4xl font-bold text-white"
                                            style="font-family: 'Chakra Petch', sans-serif">{{
                                            account.hours.toLocaleString() }} hrs</p>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-sm text-zinc-500">Account age</p>
                                        <p class="text-lg font-semibold text-white">{{ account.age }}</p>
                                    </div>
                                </div>

                                <div class="mt-6 flex h-28 items-end gap-2">
                                    <div v-for="(w, i) in account.weeks" :key="i"
                                        class="flex-1 rounded-t bg-gradient-to-t opacity-90" :class="game.bar"
                                        :style="{ height: (w / maxWeek) * 100 + '%' }" />
                                </div>

                                <div class="mt-8 space-y-5 border-t border-white/5 pt-6">
                                    <div v-for="u in account.unlocked" :key="u.label">
                                        <div class="mb-2 flex justify-between text-sm">
                                            <span class="text-zinc-300">{{ u.label }}</span>
                                            <span class="font-semibold text-white">{{ u.done }} / {{ u.total }}</span>
                                        </div>
                                        <div class="h-2 overflow-hidden rounded-full bg-white/5">
                                            <div class="h-full rounded-full bg-gradient-to-r" :class="game.bar"
                                                :style="{ width: (u.done / u.total) * 100 + '%' }" />
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="mt-14">
                            <h2 class="text-2xl font-bold text-white" style="font-family: 'Chakra Petch', sans-serif">
                                Account history</h2>
                            <ol class="mt-6 space-y-0">
                                <li v-for="(h, i) in account.history" :key="h.title"
                                    class="relative flex gap-5 pb-8 last:pb-0">
                                    <div class="flex flex-col items-center">
                                        <span
                                            class="z-10 mt-1.5 h-3 w-3 rounded-full border-2 border-[#ff6a3d] bg-[#090a0f]" />
                                        <span v-if="i < account.history.length - 1"
                                            class="mt-1 w-px flex-1 bg-white/10" />
                                    </div>
                                    <div class="-mt-0.5">
                                        <p class="text-sm text-zinc-500">{{ h.date }}</p>
                                        <h3 class="mt-0.5 font-semibold text-white">{{ h.title }}</h3>
                                        <p class="mt-1 text-zinc-400">{{ h.body }}</p>
                                    </div>
                                </li>
                            </ol>
                        </section>

                        <section class="mt-14">
                            <h2 class="text-2xl font-bold text-white" style="font-family: 'Chakra Petch', sans-serif">
                                What you receive</h2>
                            <p class="mt-3 max-w-2xl text-zinc-400">After payment you get complete control. Change the
                                email and every credential as soon as you log in and nobody else can get back in.</p>
                            <div class="mt-6 grid gap-4 sm:grid-cols-3">
                                <div v-for="h in handover" :key="h.title"
                                    class="rounded-2xl border border-white/10 bg-[#0e1017] p-5">
                                    <span
                                        class="grid h-11 w-11 place-items-center rounded-xl bg-gradient-to-br from-[#ff6a3d]/20 to-[#ff2e63]/20 text-[#ff7a54]">
                                        <Icon :name="h.icon" class="h-5 w-5" />
                                    </span>
                                    <h3 class="mt-4 font-semibold text-white">{{ h.title }}</h3>
                                    <p class="mt-1.5 text-sm leading-relaxed text-zinc-400">{{ h.body }}</p>
                                </div>
                            </div>
                        </section>
                    </article>

                    <aside id="buy" class="lg:sticky lg:top-24 lg:self-start">
                        <div class="rounded-3xl border border-white/10 bg-[#11131a] p-6 shadow-2xl">
                            <div class="flex items-start justify-between">
                                <div>
                                    <p class="text-sm text-zinc-500">Price</p>
                                    <p class="text-5xl font-bold text-white"
                                        style="font-family: 'Chakra Petch', sans-serif">${{ account.price }}</p>
                                </div>
                                <span class="rounded-full px-3 py-1 text-xs font-semibold"
                                    :class="account.stock <= 1 ? 'bg-amber-400/10 text-amber-400' : 'bg-emerald-400/10 text-emerald-400'">
                                    {{ account.stock === 1 ? 'Only 1 available' : account.stock + ' available' }}
                                </span>
                            </div>

                            <a href="#"
                                class="mt-6 flex items-center justify-center gap-2.5 rounded-xl bg-[#5865f2] px-6 py-4 font-semibold text-white transition hover:brightness-110">
                                <Icon name="simple-icons:discord" class="h-5 w-5" />
                                Buy on Discord
                            </a>
                            <a href="#"
                                class="mt-3 flex items-center justify-center gap-2.5 rounded-xl bg-[#229ed9] px-6 py-4 font-semibold text-white transition hover:brightness-110">
                                <Icon name="simple-icons:telegram" class="h-5 w-5" />
                                Buy on Telegram
                            </a>
                            <button
                                class="mt-3 flex w-full items-center justify-center gap-2.5 rounded-xl border border-white/10 bg-white/5 px-6 py-4 font-semibold text-white transition hover:bg-white/10"
                                @click="copyCode">
                                <Icon :name="copied ? 'lucide:check' : 'lucide:copy'" class="h-4 w-4" />
                                {{ copied ? 'Copied' : 'Copy account ID ' + account.code }}
                            </button>

                            <p class="mt-4 text-center text-xs text-zinc-500">Send the account ID when you message me.
                            </p>

                            <ul class="mt-6 space-y-3.5 border-t border-white/5 pt-6">
                                <li v-for="g in guarantees" :key="g.text"
                                    class="flex items-center gap-3 text-sm text-zinc-300">
                                    <Icon :name="g.icon" class="h-5 w-5 shrink-0 text-emerald-400" />
                                    {{ g.text }}
                                </li>
                            </ul>

                            <dl class="mt-6 space-y-3 border-t border-white/5 pt-6 text-sm">
                                <div class="flex justify-between">
                                    <dt class="text-zinc-500">Platform</dt>
                                    <dd class="flex items-center gap-1.5 font-medium text-white">
                                        <Icon :name="account.platform" class="h-4 w-4" />
                                        {{ account.platformName }}
                                    </dd>
                                </div>
                                <div class="flex justify-between">
                                    <dt class="text-zinc-500">Region</dt>
                                    <dd class="font-medium text-white">{{ account.region }}</dd>
                                </div>
                                <div class="flex justify-between">
                                    <dt class="text-zinc-500">Account age</dt>
                                    <dd class="font-medium text-white">{{ account.age }}</dd>
                                </div>
                                <div class="flex justify-between">
                                    <dt class="text-zinc-500">Account ID</dt>
                                    <dd class="font-medium text-white">{{ account.code }}</dd>
                                </div>
                            </dl>
                        </div>
                    </aside>
                </div>

                <section class="mt-24">
                    <div class="flex items-end justify-between">
                        <h2 class="text-2xl font-bold text-white sm:text-3xl"
                            style="font-family: 'Chakra Petch', sans-serif">More accounts</h2>
                        <NuxtLink to="/#accounts"
                            class="text-sm font-semibold text-zinc-400 transition hover:text-white">See all</NuxtLink>
                    </div>

                    <div class="mt-8 grid gap-5 sm:grid-cols-2 lg:grid-cols-3">
                        <NuxtLink v-for="r in related" :key="r.id" :to="`/accounts/${r.id}`"
                            class="group overflow-hidden rounded-3xl border border-white/10 bg-[#0e1017] transition hover:-translate-y-1 hover:border-white/20">
                            <div class="relative h-28 bg-gradient-to-br" :class="getGame(r.game).tint">
                                <div
                                    class="absolute left-5 top-5 flex items-center gap-2 rounded-full bg-black/40 px-3 py-1 text-xs font-semibold text-white backdrop-blur">
                                    <span class="h-2 w-2 rounded-full" :class="getGame(r.game).dot" />
                                    {{ getGame(r.game).name }}
                                </div>
                                <div class="absolute bottom-4 left-5 text-lg font-bold text-white"
                                    style="font-family: 'Chakra Petch', sans-serif">{{ r.hours.toLocaleString() }} hrs
                                </div>
                            </div>
                            <div class="flex items-center justify-between p-5">
                                <h3 class="font-semibold text-white">{{ r.title }}</h3>
                                <span class="text-xl font-bold text-white"
                                    style="font-family: 'Chakra Petch', sans-serif">${{ r.price }}</span>
                            </div>
                        </NuxtLink>
                    </div>
                </section>
            </div>
        </main>

        <footer class="border-t border-white/5">
            <div
                class="mx-auto flex max-w-7xl flex-col items-center justify-between gap-4 px-5 py-10 text-sm text-zinc-500 md:flex-row">
                <span class="font-semibold text-zinc-300"
                    style="font-family: 'Chakra Petch', sans-serif">respawn99</span>
                <p>© 2026 respawn99. Not affiliated with any game publisher or platform.</p>
            </div>
        </footer>
    </div>
</template>