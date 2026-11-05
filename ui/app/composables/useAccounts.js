export const gameList = [
  { id: 'rust', name: 'Rust', tint: 'from-[#ff6a3d]/30 via-[#7a2e14]/20 to-transparent', dot: 'bg-[#ff6a3d]', bar: 'from-[#ff8a5c] to-[#ff2e63]' },
  { id: 'mw', name: 'Call of Duty MW', tint: 'from-[#5fd3a2]/25 via-[#14473a]/20 to-transparent', dot: 'bg-[#5fd3a2]', bar: 'from-[#5fd3a2] to-[#2a9d8f]' },
  { id: 'wardogs', name: 'War Dogs', tint: 'from-[#f2c14e]/25 via-[#6b4d12]/20 to-transparent', dot: 'bg-[#f2c14e]', bar: 'from-[#f2c14e] to-[#e08a1e]' }
]

const gallery = [
  { label: 'Profile overview', icon: 'lucide:user-round' },
  { label: 'Playtime page', icon: 'lucide:clock' },
  { label: 'Unlocks and items', icon: 'lucide:package-open' },
  { label: 'Ban and trade status', icon: 'lucide:shield-check' }
]

const accounts = [
  {
    id: 1,
    code: 'R99-0001',
    game: 'rust',
    title: 'Full blueprint vault',
    summary: 'Every blueprint researched, a 62 skin collection and over 4,000 hours played across three years.',
    platform: 'simple-icons:steam',
    platformName: 'Steam',
    hours: 4200,
    price: 89,
    stock: 1,
    items: ['All blueprints', '62 skins', 'Clean record, 0 bans'],
    description: [
      'This account was my main Rust profile for three years. Every hour was played in normal servers, from wipe-day starts to late-wipe raids, with no boosting and no shared logins.',
      'The full blueprint tree is researched, so you can craft anything from the first minute. The skin collection was built over time through drops and purchases, and the inventory is tradable.',
      'The Steam profile is clean: no VAC bans, no game bans and no trade holds. You get complete access and can change the email and every credential straight after purchase.'
    ],
    stats: [
      { label: 'Total playtime', value: '4,200 hrs' },
      { label: 'Skins', value: '62' },
      { label: 'Steam level', value: '34' },
      { label: 'Bans', value: '0' }
    ],
    unlocked: [
      { label: 'Blueprints researched', done: 100, total: 100 },
      { label: 'Skin collection', done: 62, total: 80 },
      { label: 'Achievements', done: 41, total: 50 }
    ],
    gallery,
    history: [
      { date: '2023', title: 'Account created', body: 'Started on a fresh Steam profile with Rust purchased on day one.' },
      { date: '2024', title: 'Passed 2,000 hours', body: 'Moved into clan play and late-wipe raiding.' },
      { date: '2025', title: 'Full research tree', body: 'Finished every blueprint and built most of the skin set.' },
      { date: '2026', title: 'Listed for sale', body: 'Retired from play and listed here with the full record.' }
    ],
    weeks: [14, 22, 18, 30, 26, 34, 28, 38, 32, 41, 36, 44],
    age: '3 years',
    lastPlayed: '2 weeks ago',
    region: 'EU'
  },
  {
    id: 2,
    code: 'R99-0002',
    game: 'mw',
    title: 'Max prestige, all camos',
    summary: 'Prestige Master with Dark Matter on every weapon class and all operators unlocked.',
    platform: 'simple-icons:battledotnet',
    platformName: 'Battle.net',
    hours: 1150,
    price: 74,
    stock: 1,
    items: ['Dark Matter camos', 'All operators', 'Prestige Master'],
    description: [
      'Every camo challenge on this account was completed in real matches. The playtime is tracked in the launcher and matches the stat page, so what you see is what was played.',
      'All operators and weapon platforms are unlocked, with Dark Matter finished across each class. The account sits at Prestige Master with all milestone rewards claimed.',
      'You receive the Battle.net and Activision logins and can replace the email, password and phone number the moment you get access.'
    ],
    stats: [
      { label: 'Total playtime', value: '1,150 hrs' },
      { label: 'Prestige', value: 'Master' },
      { label: 'K/D ratio', value: '1.62' },
      { label: 'Bans', value: '0' }
    ],
    unlocked: [
      { label: 'Dark Matter weapons', done: 48, total: 48 },
      { label: 'Operators', done: 36, total: 36 },
      { label: 'Calling cards', done: 410, total: 520 }
    ],
    gallery,
    history: [
      { date: 'Launch', title: 'Account created', body: 'Registered at release with the standard edition.' },
      { date: 'Season 3', title: 'First prestige', body: 'Began the camo grind across assault rifles and SMGs.' },
      { date: 'Season 6', title: 'Gold complete', body: 'All weapon classes reached gold camos.' },
      { date: 'Season 8', title: 'Dark Matter done', body: 'Finished the final weapon and reached Prestige Master.' }
    ],
    weeks: [20, 18, 26, 22, 30, 28, 24, 32, 27, 35, 30, 38],
    age: '2 years',
    lastPlayed: '1 month ago',
    region: 'NA'
  },
  {
    id: 3,
    code: 'R99-0003',
    game: 'wardogs',
    title: 'Rank 100 loaded',
    summary: 'Max rank with every weapon unlocked, 5 million credits and premium gear.',
    platform: 'simple-icons:steam',
    platformName: 'Steam',
    hours: 380,
    price: 32,
    stock: 2,
    items: ['All weapons', '5M credits', 'Premium gear'],
    description: [
      'Played steadily and ranked up through regular matches. The hours are real and visible on the Steam profile.',
      'Every weapon is unlocked and the credit balance is ready for anything new. Premium gear sets are included.',
      'Full access is handed over, with a short guide for changing the email and password.'
    ],
    stats: [
      { label: 'Total playtime', value: '380 hrs' },
      { label: 'Rank', value: '100' },
      { label: 'Credits', value: '5M' },
      { label: 'Bans', value: '0' }
    ],
    unlocked: [
      { label: 'Weapons', done: 64, total: 64 },
      { label: 'Gear sets', done: 18, total: 24 },
      { label: 'Achievements', done: 22, total: 30 }
    ],
    gallery,
    history: [
      { date: 'Month 1', title: 'Account created', body: 'Fresh profile and first matches.' },
      { date: 'Month 4', title: 'Rank 50', body: 'Unlocked the mid-tier weapon set.' },
      { date: 'Month 8', title: 'Rank 100', body: 'Reached the cap and finished the weapon list.' }
    ],
    weeks: [8, 12, 10, 16, 14, 18, 12, 20, 15, 22, 18, 24],
    age: '1 year',
    lastPlayed: '3 weeks ago',
    region: 'Global'
  },
  {
    id: 4,
    code: 'R99-0004',
    game: 'rust',
    title: 'Rare skins starter',
    summary: 'A tidy Rust account with 18 rare skins and Prime status, ideal for a fresh start.',
    platform: 'simple-icons:steam',
    platformName: 'Steam',
    hours: 900,
    price: 28,
    stock: 1,
    items: ['18 skins', 'Steam Level 20', 'Prime status'],
    description: [
      'A lighter account with real playtime and a handful of rare skins picked for looks, not volume.',
      'Prime status is active and the profile is fully clean, with no bans or limits.',
      'Complete access is yours after purchase, and the guide walks through changing every credential.'
    ],
    stats: [
      { label: 'Total playtime', value: '900 hrs' },
      { label: 'Skins', value: '18' },
      { label: 'Steam level', value: '20' },
      { label: 'Bans', value: '0' }
    ],
    unlocked: [
      { label: 'Blueprints researched', done: 54, total: 100 },
      { label: 'Skin collection', done: 18, total: 80 },
      { label: 'Achievements', done: 24, total: 50 }
    ],
    gallery,
    history: [
      { date: '2024', title: 'Account created', body: 'Rust purchased and played from the start.' },
      { date: '2025', title: 'Prime unlocked', body: 'Verified phone number and Prime activated.' }
    ],
    weeks: [10, 14, 12, 18, 16, 20, 15, 22, 18, 24, 20, 26],
    age: '2 years',
    lastPlayed: '1 month ago',
    region: 'EU'
  },
  {
    id: 5,
    code: 'R99-0005',
    game: 'mw',
    title: 'Season grinder bundle',
    summary: 'Gold camos, a finished battle pass and three ultra skins on a level 55 profile.',
    platform: 'simple-icons:steam',
    platformName: 'Steam',
    hours: 640,
    price: 41,
    stock: 2,
    items: ['Gold camos', 'Battle pass done', '3 ultra skins'],
    description: [
      'Built across a couple of seasons of regular play. The battle pass is fully completed and the rewards are claimed.',
      'Gold camos are done on the main weapons and three ultra skins are in the collection.',
      'You get complete access and can change the email and password straight away.'
    ],
    stats: [
      { label: 'Total playtime', value: '640 hrs' },
      { label: 'Level', value: '55' },
      { label: 'K/D ratio', value: '1.28' },
      { label: 'Bans', value: '0' }
    ],
    unlocked: [
      { label: 'Gold camos', done: 32, total: 48 },
      { label: 'Battle pass tiers', done: 100, total: 100 },
      { label: 'Operators', done: 22, total: 36 }
    ],
    gallery,
    history: [
      { date: 'Season 5', title: 'Account created', body: 'Started playing seasonally.' },
      { date: 'Season 7', title: 'Battle pass done', body: 'Completed all tiers and claimed every reward.' }
    ],
    weeks: [12, 16, 14, 20, 18, 22, 17, 24, 20, 26, 22, 28],
    age: '1.5 years',
    lastPlayed: '2 weeks ago',
    region: 'NA'
  },
  {
    id: 6,
    code: 'R99-0006',
    game: 'wardogs',
    title: 'Elite arsenal account',
    summary: 'Rank 70 with legendary guns, 2 million credits and a spotless record.',
    platform: 'simple-icons:steam',
    platformName: 'Steam',
    hours: 210,
    price: 19,
    stock: 3,
    items: ['Legendary guns', '2M credits', 'No bans'],
    description: [
      'A compact account with a strong weapon set and a healthy credit balance, played normally from the start.',
      'Legendary guns are unlocked and the profile has never had a ban or restriction.',
      'Full access is handed over with a simple guide for updating the credentials.'
    ],
    stats: [
      { label: 'Total playtime', value: '210 hrs' },
      { label: 'Rank', value: '70' },
      { label: 'Credits', value: '2M' },
      { label: 'Bans', value: '0' }
    ],
    unlocked: [
      { label: 'Weapons', done: 41, total: 64 },
      { label: 'Gear sets', done: 10, total: 24 },
      { label: 'Achievements', done: 14, total: 30 }
    ],
    gallery,
    history: [
      { date: 'Month 1', title: 'Account created', body: 'Fresh profile and first ranked matches.' },
      { date: 'Month 5', title: 'Rank 70', body: 'Unlocked the legendary weapon tier.' }
    ],
    weeks: [6, 8, 7, 11, 9, 12, 8, 13, 10, 14, 11, 15],
    age: '8 months',
    lastPlayed: '1 month ago',
    region: 'Global'
  }
]

export const useAccounts = () => ({
  accounts,
  gameList,
  getAccount: (id) => accounts.find((a) => a.id === Number(id)),
  getGame: (id) => gameList.find((g) => g.id === id)
})