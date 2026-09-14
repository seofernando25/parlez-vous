import { XP_THRESHOLDS, type Tier } from './meta';

export const TIERS: Tier[] = [
    {
        level: 1,
        cefr: 'A1',
        name: 'The Outskirts',
        description: 'Survival Phrases and Basics.',
        color: 'bg-tier-1',
        xpThreshold: XP_THRESHOLDS[1],
        themes: [
            {
                id: 'greetings',
                name: 'Greetings and Introductions',
                description: 'Learn to say hello and introduce yourself.',
                subthemes: [
                    "Formal Greetings: Workplace, elders, and professional settings.",
                    "Informal Greetings: Slang, casual check-ins, and peer-to-peer.",
                    "Time of Day: Morning, afternoon, evening, and night specifics.",
                    "Farewells & Sign-offs: Goodbyes, 'see you later,' and written closings.",
                    "Personal Details: Stating your name, age, and where you are from.",
                    "Introducing Others: Presenting a friend or colleague to someone new.",
                    "'How are you?': Asking about well-being and common responses.",
                    "Titles & Honorifics: Mr., Ms., Dr., and culturally specific respect markers.",
                    "Body Language & Etiquette: Handshakes, bowing, and physical proximity.",
                    "Politeness Markers: Please, thank you, you're welcome, and apologies."
                ]
            },
            {
                id: 'numbers',
                name: 'Numbers & Currency',
                description: 'Count and handle basic transactions.',
                subthemes: [
                    "Cardinal Numbers: Counting from 1 to 1,000+.",
                    "Ordinal Numbers: 1st, 2nd, 3rd (useful for dates and ranking).",
                    "Decimals & Fractions: Half, quarter, point-five.",
                    "Basic Math: Add, subtract, multiply, and divide vocabulary.",
                    "Measurements: Weight, length, distance, and volume.",
                    "Asking for Prices: 'How much is this?' and negotiating.",
                    "Handling Transactions: Cash, credit cards, change, and receipts.",
                    "Banking Basics: ATMs, accounts, deposits, and withdrawals.",
                    "Currency Terms: Exchange rates, coins, bills, and local money slang.",
                    "Contact Info: Reading phone numbers, zip codes, and addresses aloud."
                ]
            },
        ]
    },
    {
        level: 2,
        cefr: 'A2',
        name: 'The Marketplace',
        description: 'Shopping, Food, and Routine.',
        color: 'bg-tier-2',
        xpThreshold: XP_THRESHOLDS[2],
        themes: [
            {
                id: 'family',
                name: 'Family & Friends',
                description: 'Discuss relationships and appearance.',
                subthemes: [
                    "Immediate Family: Parents, siblings, and children.",
                    "Extended Family: Grandparents, cousins, aunts, and uncles.",
                    "Relationship Status: Single, married, divorced, dating.",
                    "Physical Appearance: Describing height, hair, eye color, and build.",
                    "Personality Traits: Describing character (funny, shy, outgoing, stubborn).",
                    "Friendship Dynamics: Acquaintances, best friends, and socializing.",
                    "Pets & Animals: Types of common household pets and animal care.",
                    "Life Milestones: Births, weddings, graduations, and funerals.",
                    "Generations: Describing age groups (toddlers, teenagers, elderly).",
                    "Household Roles: Family traditions, dynamics, and responsibilities."
                ]
            },
            {
                id: 'time',
                name: 'Days, Time, & Holidays',
                description: 'Navigate schedules and calendar events.',
                subthemes: [
                    "Days of the Week: Weekdays vs. weekends.",
                    "Months & Seasons: The calendar year and seasonal changes.",
                    "Telling Time: Hours, minutes, 'quarter past,' and 'half past.'",
                    "Parts of the Day: Dawn, morning, noon, dusk, and midnight.",
                    "Time Markers: Yesterday, today, tomorrow, and 'next week.'",
                    "Frequency Adverbs: Always, sometimes, rarely, and never.",
                    "National Holidays: Major cultural and religious celebrations.",
                    "Personal Dates: Birthdays and anniversaries.",
                    "Scheduling: Making, changing, or canceling appointments.",
                    "Punctuality: Concepts of being early, on time, or running late."
                ]
            },
        ]
    },
    {
        level: 3,
        cefr: 'B1',
        name: 'The Town Square',
        description: 'Living Arrangements & Chores.',
        color: 'bg-tier-3',
        xpThreshold: XP_THRESHOLDS[3],
        themes: [
            {
                id: 'food',
                name: 'Food & Cooking',
                description: 'Discuss recipes, diets, and restaurant dining.',
                subthemes: [
                    "Ingredients: Fruits, vegetables, meats, and grains.",
                    "Meals of the Day: Breakfast, lunch, dinner, and snacks.",
                    "Cooking Verbs: Boil, fry, bake, chop, and grill.",
                    "Kitchenware: Utensils, pots, pans, and appliances.",
                    "Restaurant Dining: Ordering, asking for the menu, and tipping.",
                    "Taste & Texture: Sweet, sour, spicy, salty, bitter, and crunchy.",
                    "Dietary Needs: Allergies, vegetarian/vegan, and gluten-free.",
                    "Recipes: Reading instructions, portions, and measurements.",
                    "Grocery Shopping: Navigating the supermarket and reading labels.",
                    "Food Culture: Traditional dishes, street food, and dining etiquette."
                ]
            },
            {
                id: 'house',
                name: 'Around the House',
                description: 'Household chores, furniture, and daily routines.',
                subthemes: [
                    "Rooms of the House: Kitchen, bedroom, bathroom, living room.",
                    "Types of Housing: Apartments, houses, dorms, and countryside villas.",
                    "Furniture & Decor: Beds, sofas, tables, rugs, and lighting.",
                    "Household Chores: Sweeping, laundry, washing dishes, and taking out trash.",
                    "Daily Routines: Waking up, getting dressed, and bedtime habits.",
                    "Personal Hygiene: Showers, brushing teeth, and bathroom supplies.",
                    "Home Maintenance: Fixing leaks, changing bulbs, and basic repairs.",
                    "Real Estate: Renting, buying, landlords, and leases.",
                    "Outdoor Spaces: Gardens, balconies, patios, and garages.",
                    "Utilities: Electricity, water, heating, AC, and internet."
                ]
            },
        ]
    },
    {
        level: 4,
        cefr: 'B2',
        name: 'The Library',
        description: 'City Life & Hobbies.',
        color: 'bg-tier-4',
        xpThreshold: XP_THRESHOLDS[4],
        themes: [
            {
                id: 'city',
                name: 'City Life & Directions',
                description: 'Navigate urban environments and public transit.',
                subthemes: [
                    "Public Transit: Buses, trains, subways, and trams.",
                    "Giving Directions: Turn left, go straight, cross the street.",
                    "Prepositions of Place: Next to, behind, across from, and between.",
                    "Urban Landmarks: Banks, post offices, hospitals, and parks.",
                    "Street Infrastructure: Sidewalks, intersections, traffic lights, and bridges.",
                    "Ticketing: Buying passes, checking schedules, and fares.",
                    "Driving & Traffic: Cars, parking, speed limits, and traffic jams.",
                    "Neighborhoods: Downtown, suburbs, residential, and commercial zones.",
                    "Safety & Emergencies: Police, fire department, and reporting theft.",
                    "Map Reading: Understanding cardinal directions (North, South, East, West)."
                ]
            },
            {
                id: 'hobbies',
                name: 'Free Time & Hobbies',
                description: 'Discuss leisure activities and interests.',
                subthemes: [
                    "Sports & Athletics: Soccer, basketball, swimming, and matches.",
                    "Outdoor Recreation: Hiking, camping, cycling, and fishing.",
                    "Indoor Gaming: Board games, card games, and video games.",
                    "Arts & Crafts: Painting, drawing, knitting, and woodworking.",
                    "Music: Playing instruments, singing, and reading sheet music.",
                    "Literature: Reading books, comics, magazines, and writing.",
                    "Fitness: Gym vocabulary, yoga, weightlifting, and stretching.",
                    "Collecting: Stamps, coins, antiques, and memorabilia.",
                    "Gardening: Plant care, growing vegetables, and landscaping.",
                    "Clubs & Groups: Joining local societies, volunteering, and meetups."
                ]
            },
        ]
    }
];
