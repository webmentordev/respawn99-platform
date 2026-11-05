<template>
    <header class="sticky top-0 z-50 border-b border-white/5 bg-ink/80 backdrop-blur-xl">
        <div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-5">
            <a href="#" class="flex items-center gap-2.5">
                <span class="grid h-8 w-8 place-items-center rounded-lg bg-gradient-to-br from-ember to-hot text-white">
                    <Icon name="lucide:crosshair" class="h-4.5 w-4.5" />
                </span>
                <span class="text-lg font-semibold tracking-tight text-white font-display">respawn99</span>
            </a>

            <nav class="hidden items-center gap-8 text-sm font-medium md:flex">
                <a href="#games" class="transition hover:text-white">Games</a>
                <a href="#accounts" class="transition hover:text-white">Accounts</a>
                <a href="#playtime" class="transition hover:text-white">Playtime</a>
                <a href="#access" class="transition hover:text-white">Full access</a>
                <a href="#faq" class="transition hover:text-white">FAQ</a>
            </nav>

            <div class="hidden items-center gap-3 md:flex">
                <a href="#contact"
                    class="inline-flex items-center gap-2 rounded-lg px-4 py-2 text-sm font-semibold text-zinc-300 transition hover:text-white">
                    <Icon name="simple-icons:discord" class="h-4 w-4" />
                    Chat with me
                </a>
                <a href="#accounts"
                    class="rounded-lg bg-white px-4 py-2 text-sm font-semibold text-black transition hover:bg-zinc-200">Browse
                    accounts</a>
            </div>

            <button class="grid h-10 w-10 place-items-center rounded-lg border border-white/10 md:hidden"
                aria-label="Toggle menu" @click="menu = !menu">
                <Icon :name="menu ? 'lucide:x' : 'lucide:menu'" class="h-5 w-5" />
            </button>
        </div>

        <div v-if="menu" class="border-t border-white/5 px-5 pb-5 pt-3 md:hidden">
            <div class="flex flex-col gap-1 text-sm font-medium">
                <a href="#games" class="rounded-lg px-3 py-2.5 hover:bg-white/5" @click="menu = false">Games</a>
                <a href="#accounts" class="rounded-lg px-3 py-2.5 hover:bg-white/5" @click="menu = false">Accounts</a>
                <a href="#playtime" class="rounded-lg px-3 py-2.5 hover:bg-white/5" @click="menu = false">Playtime</a>
                <a href="#access" class="rounded-lg px-3 py-2.5 hover:bg-white/5" @click="menu = false">Full
                    access</a>
                <a href="#faq" class="rounded-lg px-3 py-2.5 hover:bg-white/5" @click="menu = false">FAQ</a>
                <a href="#contact" class="mt-2 rounded-lg bg-white px-4 py-3 text-center font-semibold text-black"
                    @click="menu = false">Chat with me</a>
            </div>
        </div>
    </header>
</template>

<script setup lang="js">
const menu = ref(false)
</script>