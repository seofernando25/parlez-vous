import { XP_THRESHOLDS, type Tier } from './meta';

export const TIERS: Tier[] = [
    {
        level: 5,
        cefr: 'C1',
        name: 'The University',
        description: 'Careers, Health, & Travel.',
        color: 'bg-tier-5',
        xpThreshold: XP_THRESHOLDS[5],
        themes: [
            {
                id: 'career',
                name: 'Jobs and Careers',
                description: 'Professional environments and workplace culture.',
                subthemes: [
                    "Common Professions: Teacher, engineer, artist, chef, etc.",
                    "The Job Hunt: Resumes, cover letters, and interviews.",
                    "Workplace Environment: Colleagues, bosses, and corporate hierarchy.",
                    "Office Vocabulary: Desks, meetings, presentations, and supplies.",
                    "Compensation: Salaries, hourly wages, taxes, and benefits.",
                    "Daily Tasks: Managing emails, answering calls, and project deadlines.",
                    "Employment Status: Hiring, firing, quitting, and retiring.",
                    "Entrepreneurship: Starting a business, clients, and freelancing.",
                    "Remote Work: Video calls, digital collaboration, and working from home.",
                    "Work Culture: Professional etiquette, water-cooler talk, and networking."
                ]
            },
            {
                id: 'health',
                name: 'Health and the Body',
                description: 'Medical vocabulary, wellness, and fitness.',
                subthemes: [
                    "Human Anatomy: External body parts and internal organs.",
                    "Common Illnesses: Colds, flus, headaches, and stomachaches.",
                    "Symptoms: Coughing, sneezing, fever, and nausea.",
                    "Medical Care: Visiting the doctor, hospitals, and clinics.",
                    "Pharmacy: Prescriptions, painkillers, vitamins, and bandages.",
                    "First Aid: Treating cuts, burns, sprains, and calling for an ambulance.",
                    "Mental Health: Stress, anxiety, therapy, and self-care.",
                    "Specialists: Dentists, optometrists, surgeons, and dermatologists.",
                    "Healthy Lifestyle: Nutrition, dieting, and sleep hygiene.",
                    "Physical Therapy: Recovery, mobility, and chronic pain management."
                ]
            },
            {
                id: 'travel',
                name: 'Travel & Tourism',
                description: 'Vacations, accommodations, and airports.',
                subthemes: [
                    "Airport Navigation: Check-in, security, gates, and boarding.",
                    "Accommodations: Booking hotels, hostels, and vacation rentals.",
                    "Border Control: Passports, visas, customs, and immigration.",
                    "Luggage: Packing, suitcases, carry-ons, and baggage claim.",
                    "Modes of Travel: Airplanes, cruise ships, long-distance buses.",
                    "Sightseeing: Museums, historical sites, and tour guides.",
                    "Travel Delays: Cancellations, layovers, and lost luggage.",
                    "Souvenirs: Local markets, haggling, and cultural gifts.",
                    "Travel Documents: Tickets, itineraries, and travel insurance.",
                    "Reviews & Recommendations: Writing postcards, reviewing trips, and rating hotels."
                ]
            },
        ]
    },
    {
        level: 6,
        cefr: 'C2',
        name: 'The Capital',
        description: 'Arts, Story-Telling, Technology.',
        color: 'bg-tier-6',
        xpThreshold: XP_THRESHOLDS[6],
        themes: [
            {
                id: 'arts',
                name: 'The Arts & Media',
                description: 'Movies, music, literature, and critique.',
                subthemes: [
                    "Cinema: Movie genres, directors, actors, and screenplays.",
                    "Music Industry: Concerts, albums, genres, and streaming.",
                    "Theater & Dance: Live plays, musicals, ballet, and choreography.",
                    "Visual Arts: Museums, exhibitions, sculpture, and photography.",
                    "Literature & Poetry: Novels, authors, literary devices, and publishing.",
                    "Television: Series, broadcasting, reality TV, and documentaries.",
                    "Journalism: Newspapers, reporters, headlines, and bias.",
                    "Social Media: Influencers, viral content, algorithms, and digital culture.",
                    "Critique & Review: Expressing opinions, analyzing themes, and rating art.",
                    "Cultural Heritage: Folklore, mythology, and historical artifacts."
                ]
            },
            {
                id: 'technology',
                name: 'Technology & Science',
                description: 'Discuss modern advancements and digital life.',
                subthemes: [
                    "Hardware: Computers, smartphones, processors, and screens.",
                    "Software & Apps: Operating systems, coding, and user interfaces.",
                    "The Internet: Browsers, Wi-Fi, cloud storage, and connectivity.",
                    "Cybersecurity: Passwords, hacking, viruses, and data privacy.",
                    "Artificial Intelligence: Machine learning, robotics, and automation.",
                    "Space Exploration: Planets, astronomy, satellites, and rockets.",
                    "Basic Sciences: Physics, chemistry, biology, and the scientific method.",
                    "Research & Labs: Experiments, data analysis, and publishing papers.",
                    "Gadgets & Wearables: Smartwatches, VR, and home automation.",
                    "Future Innovations: Quantum computing, bioengineering, and nanotech."
                ]
            },
        ]
    },
    {
        level: 7,
        cefr: 'Mastery',
        name: 'The Horizon',
        description: 'Environment & Current Events.',
        color: 'bg-tier-7',
        xpThreshold: XP_THRESHOLDS[7],
        themes: [
            {
                id: 'environment',
                name: 'The Environment',
                description: 'Climate change, nature, and conservation.',
                subthemes: [
                    "Climate Change: Global warming, greenhouse gases, and carbon footprints.",
                    "Energy Sources: Renewable (solar/wind) vs. fossil fuels (oil/coal).",
                    "Pollution: Air, water, soil contamination, and smog.",
                    "Waste Management: Recycling, composting, landfills, and plastics.",
                    "Natural Disasters: Earthquakes, hurricanes, wildfires, and floods.",
                    "Ecosystems: Oceans, rainforests, deserts, and biodiversity.",
                    "Wildlife Conservation: Endangered species, poaching, and habitats.",
                    "Agriculture: Factory farming, organic growing, and pesticides.",
                    "Geography: Mountains, rivers, continents, and topography.",
                    "Meteorology: Extreme weather patterns, forecasting, and atmospheric science."
                ]
            },
            {
                id: 'news',
                name: 'Current Events',
                description: 'Global politics, economy, and society.',
                subthemes: [
                    "Global Politics: Elections, governments, democracies, and dictatorships.",
                    "Economics: Inflation, stock markets, trade agreements, and poverty.",
                    "International Relations: Diplomacy, treaties, the UN, and foreign policy.",
                    "Social Justice: Human rights, protests, equality, and systemic issues.",
                    "Public Health: Pandemics, global aid, and healthcare access.",
                    "Law & Justice: Courts, crime, legislation, and the penal system.",
                    "Migration: Refugees, borders, immigration policy, and citizenship.",
                    "Labor Movements: Strikes, unions, workers' rights, and wage gaps.",
                    "Education Policy: Access to schooling, university funding, and literacy.",
                    "Philanthropy: NGOs, charities, disaster relief, and volunteerism."
                ]
            },
        ]
    }
];
