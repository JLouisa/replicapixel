import { PrismaClient } from "./generated/prisma";
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
  await prisma.users.upsert({
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
            content_type: "Jpeg",
            status: "Completed",
            image_size: "Square",
            image_url_fal: `https://flowbite.s3.amazonaws.com/docs/gallery/square/image-${i}.jpg`,
            is_favorite: randomBoolean,
            deleted_at: Math.random() < 0.1 ? new Date() : null,
          },
        });
      }
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
