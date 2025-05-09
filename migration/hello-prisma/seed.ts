import { FeatureStatus, PrismaClient } from "./generated/prisma";
import { v4 as uuidv4 } from "uuid";
import { faker } from "@faker-js/faker";

const prisma = new PrismaClient();

export default prisma;

async function main() {
  let user = await prisma.users.upsert({
    where: { email: "golden_dragon88@hotmail.com" },
    update: {},
    create: {
      pid: "ab5e796c-a2cd-458e-ad6b-c3a898f44bd1",
      name: "Golden",
      email: "golden_dragon88@hotmail.com",
      password:
        "$argon2id$v=19$m=19456,t=2,p=1$evLarirkLuzvgfh+jm9raQ$15L06vovy3DpL4j8IEAzGT5PrZbp+vKOSOFU/Jt1QTA",
      api_key: "123e4567-e89b-12d3-a456-426655440000",
    },
  });
  let user2 = await prisma.users.upsert({
    where: { email: "golden@example.com" },
    update: {},
    create: {
      pid: uuidv4(),
      name: "Yogurtt",
      email: "golden@example.com",
      password:
        "$argon2id$v=19$m=19456,t=2,p=1$evLarirkLuzvgfh+jm9raQ$15L06vovy3DpL4j8IEAzGT5PrZbp+vKOSOFU/Jt1QTA",
      api_key: "123e4567-e89b-12d3-a456-426655440012",
    },
  });
  await prisma.userCredits.upsert({
    where: { user_id: user.id },
    update: {},
    create: {
      pid: uuidv4(),
      user_id: user.id,
      credit_amount: 1000,
    },
  });
  await prisma.userCredits.upsert({
    where: { user_id: user2.id },
    update: {},
    create: {
      pid: uuidv4(),
      user_id: user2.id,
      credit_amount: 80,
    },
  });

  await prisma.userSettings.upsert({
    where: { user_id: user.id },
    update: {},
    create: {
      user_id: user.id,
      enable_notification_email: true,
      enable_marketing_email: false,
      language: "English",
      theme: "Dark",
    },
  });

  await prisma.userSettings.upsert({
    where: { user_id: user2.id },
    update: {},
    create: {
      user_id: user2.id,
      enable_notification_email: false,
      enable_marketing_email: true,
      language: "English",
    },
  });

  const trainingModel = await prisma.trainingModels.create({
    data: {
      pid: "5614eea0-faf6-446f-b2d4-8edfd0624bd1",
      user_id: user.id,
      name: "Adam88",
      age: 35,
      sex: "Male",
      ethnicity: "Black",
      basedOn: "RealPerson",
      eye_color: "Brown",
      bald: true,
      steps: 50,
      create_mask: false,
      is_style: false,
      trigger_word: "Adam88-b11k0m4mcy",
      tensor_path:
        "https://v3.fal.media/files/monkey/NAZ2IXZTtYeWC7h4tfBSg_pytorch_lora_weights.safetensors",
      training_status: "Completed",
      fal_ai_request_id: "2b37f874-a5d0-48f9-829d-3fdf36c12f12",
      s3_key: "ab5e796c-a2cd-458e-ad6b-c3a898f44bd1/zip/Adam88-b11k0m4mcy",
      is_verified: true,
    },
  });
  await prisma.trainingModels.create({
    data: {
      pid: "68651367-da2f-4b4e-b79e-db67b617b80b",
      user_id: user.id,
      name: "Jonathan",
      age: 35,
      sex: "Male",
      ethnicity: "Black",
      basedOn: "RealPerson",
      eye_color: "Brown",
      bald: true,
      steps: 50,
      create_mask: false,
      is_style: false,
      trigger_word: "Jonathan-i6dngv2jqj",
      training_status: "Training",
      fal_ai_request_id: "2b37f874-a5d0-48f9-829d-3fdf36c12f12",
      s3_key: "ab5e796c-a2cd-458e-ad6b-c3a898f44bd1/zip/Jonathan-i6dngv2jqj.zip",
      is_verified: true,
    },
  });
  {
    const name = faker.person.fullName();
    const uuid = uuidv4();
    const tw = `${name}-${faker.word.words(1)}`;
    await prisma.trainingModels.upsert({
      where: { pid: uuid }, // Ensure uniqueness
      update: {}, // No changes if record exists
      create: {
        pid: uuid,
        user_id: user.id,
        name: name,
        age: 35,
        sex: "Female",
        ethnicity: "White",
        basedOn: "RealPerson",
        eye_color: "Blue",
        bald: false,
        steps: 28,
        create_mask: false,
        is_style: false,
        trigger_word: tw,
        training_status: "Pending",
        fal_ai_request_id: uuidv4(),
        s3_key: `${uuid}/zip/${tw}.zip`,
        is_verified: true,
      },
    });
  }
  {
    const name = faker.person.fullName();
    const uuid = uuidv4();
    const tw = `${name}-${faker.word.words(1)}`;
    await prisma.trainingModels.upsert({
      where: { pid: uuid }, // Ensure uniqueness
      update: {}, // No changes if record exists
      create: {
        pid: uuid,
        user_id: user.id,
        name: name,
        age: 35,
        sex: "Female",
        ethnicity: "White",
        basedOn: "RealPerson",
        eye_color: "Blue",
        bald: false,
        steps: 28,
        create_mask: false,
        is_style: false,
        trigger_word: tw,
        training_status: "Failed",
        fal_ai_request_id: uuidv4(),
        s3_key: `${uuid}/zip/${tw}.zip`,
        is_verified: true,
      },
    });
  }
  {
    for (let i = 0; i <= 3; i++) {
      for (let i = 1; i <= 11; i++) {
        const randomBoolean = Math.random() > 0.25;
        const uuid = uuidv4();
        const prompt = faker.lorem.sentence(8);
        await prisma.images.upsert({
          where: { pid: uuid },
          update: {},
          create: {
            pid: uuid,
            user_id: user.id,
            training_model_id: trainingModel.id,
            alt: `Image of ${prompt}`,
            user_prompt: prompt,
            sys_prompt: prompt,
            num_inference_steps: 50,
            content_type: "jpg",
            status: "Completed",
            image_size: "Square",
            image_s3_key: `${user.pid}/images/${uuid}.jpg`,
            image_url_fal: `https://flowbite.s3.amazonaws.com/docs/gallery/square/image-${i}.jpg`,
            is_favorite: randomBoolean,
            deleted_at: Math.random() < 0.1 ? new Date() : null,
          },
        });
      }
    }
  }
  {
    const uuid = uuidv4();
    await prisma.plans.upsert({
      where: { pid: uuid }, // Ensure uniqueness
      update: {}, // No changes if record exists
      create: {
        pid: uuid,
        name: "Basic",
        plan_name: "Basic",
        credit_amount: 50,
        model_amount: 1,
        price_cents: 999,
        stripe_price_id: "price_1RAyvzHsQkT5gWXDZVsbR8VM",
        subtitle: "For individuals",
        features: [
          "60 AI Photos (credits)",
          "1 AI Model",
          "No monthly subscription!",
          "Use any photo pack",
          "No Watermarked photos",
          "24/7 Support",
        ],
        cta: "Choose Basic",
      },
    });
  }
  {
    const uuid = uuidv4();
    await prisma.plans.upsert({
      where: { pid: uuid },
      update: {},
      create: {
        pid: uuid,
        name: "Premium",
        plan_name: "Premium",
        credit_amount: 250,
        model_amount: 7,
        price_cents: 3999,
        stripe_price_id: "price_1RAywNHsQkT5gWXDGEb6Pmoi",
        subtitle: "For professionals",
        features: [
          "300 AI Photos (credits)",
          "7 AI Model",
          "No monthly subscription!",
          "Use any photo pack",
          "No Watermarked photos",
          "24/7 Support",
        ],
        cta: "Choose Premium",
      },
    });
  }
  {
    const uuid = uuidv4();
    await prisma.plans.upsert({
      where: { pid: uuid },
      update: {},
      create: {
        pid: uuid,
        name: "Max",
        plan_name: "Max",
        credit_amount: 1100,
        model_amount: 16,
        price_cents: 9999,
        stripe_price_id: "price_1RAyxOHsQkT5gWXDxQklgtPM",
        subtitle: "For Business",
        features: [
          "1100 AI Photos (credits)",
          "16 AI Model",
          "No monthly subscription!",
          "Use any photo pack",
          "No Watermarked photos",
          "24/7 Support",
        ],
        cta: "Choose Max",
      },
    });
  }
  {
    const uuid = uuidv4();
    await prisma.packs.upsert({
      where: { pid: uuid },
      update: {},
      create: {
        id: 0,
        pid: uuid,
        title: "Sexy Valentine Photos",
        short_description:
          "Step into a flirty, love-drenched fantasy this Valentine’s with an AI photo pack that lets you become your most enchanting self—no studio needed.",
        full_description:
          "This Valentine’s Day, skip the awkward photo sessions and step into a world of romantic elegance—digitally. Build your AI twin and create captivating, love-themed portraits filled with charm, style, and just the right hint of seduction. All it takes is your phone or laptop to unlock dozens of dreamy photo packs in a range of poses, moods, and settings.",
        pack_prompts:
          "Valentine's Day fantasy portrait of a stunning woman in elegant, heart-inspired attire, softly glowing in warm pink and red tones. She poses gracefully with natural limb proportions both arms and legs fully visible and correctly formed—while surrounded by romantic elements like rose petals, silky fabrics, and delicate lighting. The setting is playful yet classy, evoking charm, allure, and a modern fairytale vibe.",
        credits: 100,
        amount: 20,
        image_url: "https://picsum.photos/id/31/400/500",
      },
    });
  }
  {
    const uuid = uuidv4();
    await prisma.packs.upsert({
      where: { pid: uuid },
      update: {},
      create: {
        pid: uuid,
        title: "Sexy Halloween AI Photos",
        short_description:
          "Unleash your sultry side this Halloween by becoming the star of your own spellbinding AI photo shoot—no broomstick or booking fee required.",
        full_description:
          "This Halloween, let your inner enchantress run wild with AI-generated looks that are equal parts spooky and stunning. From flirtatious witches to dark-hearted vamps, conjure endless photo variations that look just like you crafted from your personal AI model. No need for costumes or a costly photographer just your phone, your vibe, and a little digital magic.",
        pack_prompts:
          "A mysterious and seductive Halloween portrait of a confident woman in a stylish, spooky costume such as a modern witch, vampire, or dark fairy with clearly visible and correctly proportioned arms and legs. The scene includes playful, elegant Halloween elements like soft candlelight, lace veils, smoky shadows, or glowing pumpkins, with a polished, cinematic look. She poses gracefully with natural body structure, ensuring hands, feet, and facial features are fully visible and beautifully rendered.",
        credits: 75,
        amount: 20,
        image_url: "https://picsum.photos/id/65/400/500",
      },
    });
  }
  {
    const uuid = uuidv4();
    await prisma.packs.upsert({
      where: { pid: uuid },
      update: {},
      create: {
        pid: uuid,
        title: "Sexy Easter AI Photos",
        short_description:
          "Celebrate Easter with a cheeky twist step into pastel dreamscapes as your AI twin brings bunny-eared charm and springtime seduction to life.",
        full_description:
          "This Easter, trade candy baskets for curves and charm with an AI photo shoot that blends playful innocence with sultry flair. Create your own digital twin and explore enchanting scenes filled with soft pastels, bunny-inspired accessories, and vibrant spring vibes. No studio, no stress just you, your phone, and dozens of stylish, flirty photo packs at your fingertips.",
        pack_prompts:
          "A playful and seductive Easter-themed portrait of a confident woman in pastel-colored lingerie or stylish bunny-inspired outfit, complete with fluffy bunny ears and soft accessories. She poses gracefully in a vibrant spring setting with decorated Easter eggs, blooming flowers, and gentle lighting arms, hands, legs, and feet clearly visible and naturally proportioned. The composition is festive and flirty, balancing elegance with cheeky charm, and rendered in a soft, cinematic style.",
        credits: 65,
        amount: 20,
        image_url: "https://picsum.photos/id/103/400/500",
      },
    });
  }
  {
    const uuid = uuidv4();
    await prisma.packs.upsert({
      where: { pid: uuid },
      update: {},
      create: {
        pid: uuid,
        title: "Cyberpunk Boudoir AI Photos",
        short_description:
          "Step into a neon-lit fantasy where edgy fashion meets sultry elegance—your AI twin rocks cyberpunk couture and bold boudoir looks with effortless allure.",
        full_description:
          "Slip into a seductive digital wardrobe where cyberpunk edge meets vaporwave dreams, no goggles or green screens required. With your personal AI model, you can generate striking, futuristic boudoir photos that blend glowing neon aesthetics with sensual fashion. All from your phone or laptop, no studio, no stylist, just pure pixel-powered confidence.",
        pack_prompts:
          "A futuristic boudoir portrait of a confident woman in high-tech cyberpunk lingerie or neon-trimmed bodysuits, set against a glowing cityscape or holographic interior. She poses with grace and poise, with all limbs, arms, hands, legs, and feet clearly visible and anatomically correct. The scene features vaporwave elements like grid lines, soft purples and blues, chrome textures, and ambient neon lighting, blending sensuality with sleek, digital sophistication.",
        credits: 65,
        amount: 20,
        image_url: "https://picsum.photos/id/103/400/500",
      },
    });
  }
  {
    const baseTime = new Date();

    // === Seed Feature Requests ===
    const featureRequests = [
      {
        id: 1,
        user_id: 1,
        title: "Sexy Valentine's Day",
        description: "Generate images for a spicy Valentine's Day surprise.",
        status: FeatureStatus.Suggested,
        votes: 25,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 2,
        user_id: 2,
        title: "Valentine's Day",
        description: "Create romantic imagery suitable for Valentine's Day cards or messages.",
        status: FeatureStatus.Planned,
        votes: 14,
        created_at: baseTime,
        updated_at: new Date(), // later time
      },
      {
        id: 3,
        user_id: 1,
        title: "Tinder Profile Pics",
        description: "Generate profile pictures optimized for dating apps like Tinder.",
        status: FeatureStatus.Rejected,
        votes: 6,
        created_at: baseTime,
        updated_at: baseTime,
      },

      {
        id: 4,
        user_id: 1,
        title: "Professional Head shots",
        description: "Create professional-looking head shots for LinkedIn or corporate use.",
        status: FeatureStatus.In_progress,
        votes: 1,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 5,
        user_id: 2,
        title: "Graduation Themes",
        description:
          "Design AI-generated images for graduation parties, caps, gowns, and celebration themes.",
        status: FeatureStatus.Suggested,
        votes: 9,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 6,
        user_id: 2,
        title: "AI Pets",
        description:
          "Allow users to create realistic images of imaginary pets or enhance real pet photos.",
        status: FeatureStatus.Planned,
        votes: 18,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 7,
        user_id: 1,
        title: "Fantasy Armor Sets",
        description:
          "Let users generate epic armor looks for fantasy roleplay and cosplay inspiration.",
        status: FeatureStatus.In_progress,
        votes: 34,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 8,
        user_id: 2,
        title: "AI Band Posters",
        description:
          "Generate creative concert posters or album covers for bands or musical artists.",
        status: FeatureStatus.Completed,
        votes: 22,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 9,
        user_id: 2,
        title: "Baby Announcement Themes",
        description: "Create heartwarming or fun AI images for baby announcements.",
        status: FeatureStatus.Suggested,
        votes: 13,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 10,
        user_id: 2,
        title: "Fitness Transformation Templates",
        description: "Before-and-after image tools or motivational posters with AI help.",
        status: FeatureStatus.Rejected,
        votes: 5,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 11,
        user_id: 1,
        title: "Wedding Storybooks",
        description: "Turn wedding photos into romantic AI-generated storybook scenes.",
        status: FeatureStatus.Planned,
        votes: 28,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 12,
        user_id: 1,
        title: "Sci-Fi Portraits",
        description:
          "Generate portraits inspired by science fiction aesthetics, like Star Wars or Blade Runner.",
        status: FeatureStatus.Completed,
        votes: 41,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 13,
        user_id: 2,
        title: "Cartoon Avatars",
        description:
          "Allow users to create fun cartoon versions of themselves in different styles.",
        status: FeatureStatus.In_progress,
        votes: 37,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 14,
        user_id: 1,
        title: "Book Cover Generator",
        description: "Give authors tools to generate stunning book cover visuals.",
        status: FeatureStatus.Suggested,
        votes: 19,
        created_at: baseTime,
        updated_at: baseTime,
      },
    ];

    for (const fr of featureRequests) {
      await prisma.featureRequest.upsert({
        where: { id: fr.id },
        update: {},
        create: {
          id: fr.id,
          user_id: fr.user_id,
          title: fr.title,
          description: fr.description,
          status: fr.status,
          votes: fr.votes,
          created_at: fr.created_at,
          updated_at: fr.updated_at,
        },
      });
    }

    // === Seed Feature Votes for User ID 1 ===
    const featureVotes = [
      {
        id: 1,
        user_id: 1,
        feature_request_id: 1,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 2,
        user_id: 1,
        feature_request_id: 3,
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 3,
        user_id: 1,
        feature_request_id: 6, // non-existent feature
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 4,
        user_id: 1,
        feature_request_id: 5, // "Graduation Themes"
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 5,
        user_id: 1,
        feature_request_id: 7, // "Fantasy Armor Sets"
        created_at: baseTime,
        updated_at: baseTime,
      },
      {
        id: 6,
        user_id: 2,
        feature_request_id: 7, // "Fantasy Armor Sets"
        created_at: baseTime,
        updated_at: baseTime,
      },
    ];

    for (const vote of featureVotes) {
      await prisma.featureVote.upsert({
        where: { id: vote.id },
        update: {},
        create: vote,
      });
    }
  }
}

main()
  .catch((e) => {
    console.error("❌ Seeding failed:", e);
    process.exit(1);
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
