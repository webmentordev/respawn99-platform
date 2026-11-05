-- Add migration script here
INSERT INTO
    platforms (slug, name, icon, sort_order)
VALUES (
        'steam',
        'Steam',
        'simple-icons:steam',
        1
    ),
    (
        'activision',
        'Activision',
        'simple-icons:activision',
        2
    ),
    (
        'battlenet',
        'Battle.net',
        'simple-icons:battledotnet',
        3
    ),
    (
        'xbox',
        'Xbox',
        'simple-icons:xbox',
        4
    ),
    (
        'playstation',
        'PlayStation',
        'simple-icons:playstation',
        5
    ),
    (
        'epic',
        'Epic Games',
        'simple-icons:epicgames',
        6
    ),
    (
        'ea',
        'EA',
        'simple-icons:ea',
        7
    ),
    (
        'ubisoft',
        'Ubisoft',
        'simple-icons:ubisoft',
        8
    ),
    (
        'riot',
        'Riot Games',
        'simple-icons:riotgames',
        9
    ),
    (
        'gog',
        'GOG',
        'simple-icons:gogdotcom',
        10
    ),
    (
        'nintendo',
        'Nintendo',
        'simple-icons:nintendoswitch',
        11
    ),
    (
        'rockstar',
        'Rockstar',
        'simple-icons:rockstargames',
        12
    );

INSERT INTO
    games (
        slug,
        name,
        tag,
        tint,
        ring,
        dot,
        sort_order
    )
VALUES (
        'rust',
        'Rust',
        'Leveled by our team, skins and blueprints earned in raids',
        'from-ember/30 via-rust-deep/20 to-transparent',
        'hover:border-ember/60',
        'bg-ember',
        1
    ),
    (
        'mw',
        'Call of Duty MW',
        'Levels, camos and prestige unlocked by our squad',
        'from-mw/25 via-mw-deep/20 to-transparent',
        'hover:border-mw/60',
        'bg-mw',
        2
    ),
    (
        'wardogs',
        'War Dogs',
        'Ranked up together with gear and currency',
        'from-wardogs/25 via-wardogs-deep/20 to-transparent',
        'hover:border-wardogs/60',
        'bg-wardogs',
        3
    ),
    (
        'fortnite',
        'Fortnite',
        'High level accounts with skins, emotes and battle passes',
        'from-violet-500/25 via-violet-900/20 to-transparent',
        'hover:border-violet-500/60',
        'bg-violet-500',
        4
    );

INSERT INTO
    other_games (name, sort_order)
VALUES ('Apex Legends', 1),
    ('Valorant', 2),
    ('Counter-Strike 2', 3),
    ('GTA V Online', 4),
    ('Warzone', 5),
    ('Rainbow Six Siege', 6),
    ('Minecraft', 7),
    ('Roblox', 8),
    ('League of Legends', 9),
    ('Overwatch 2', 10);

INSERT INTO
    faqs (question, answer, sort_order)
VALUES (
        'Is this a marketplace with many sellers?',
        'No. respawn99 is a team of friends who play these games together. Every account here was leveled by us and we handle every order ourselves.',
        1
    ),
    (
        'Do you sell other games?',
        'Yes. Besides Rust, Call of Duty MW, War Dogs and Fortnite, we level accounts for Apex Legends, Valorant, CS2, GTA V and more. Message us with the game you want.',
        2
    ),
    (
        'How do I know the accounts were played legit?',
        'Each account has tracked playtime, level and progress. We share screenshots of play history and unlock stats before you buy.',
        3
    ),
    (
        'Do I really get full access?',
        'Yes. You get the original email, password and recovery details, plus a step-by-step guide to change them on each platform.',
        4
    ),
    (
        'How fast is delivery?',
        'Usually within minutes of payment. If we are offline, it is delivered as soon as we are back, never later than a few hours.',
        5
    ),
    (
        'What if something goes wrong after I buy?',
        'Message us. If the account has an issue that existed before the sale, we will fix it or replace it.',
        6
    ),
    (
        'What payments do you accept?',
        'Visa, Mastercard, Apple Pay, Google Pay, USDT, Bitcoin and Litecoin.',
        7
    );