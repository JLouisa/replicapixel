// prisma/export_translations.ts
import { PrismaClient } from "./generated/prisma";
import fs from "fs";
import path from "path";

const prisma = new PrismaClient();

async function exportTranslationsToJSON() {
  // Retrieve all Plans with their translations
  const plans = await prisma.plans.findMany({});

  // Retrieve all Packs with their translations
  const packs = await prisma.packs.findMany({});

  // Extract just the translation records
  const plansTranslations = plans.map((translation) => ({
    plan_id: translation.id,
    language: "English",
    name: translation.name,
    subtitle: translation.subtitle,
    features: translation.features,
    cta: translation.cta,
  }));

  const packsTranslations = packs.map((translation) => ({
    pack_id: translation.id,
    language: "English",
    title: translation.title,
    short_description: translation.short_description,
    full_description: translation.full_description,
    features: translation.features,
  }));

  // Save to files
  fs.writeFileSync(
    path.resolve(__dirname, "plans_translations.json"),
    JSON.stringify(plansTranslations, null, 2)
  );

  fs.writeFileSync(
    path.resolve(__dirname, "packs_translations.json"),
    JSON.stringify(packsTranslations, null, 2)
  );

  console.log("Export finished");
}

exportTranslationsToJSON()
  .then(async () => {
    await prisma.$disconnect();
  })
  .catch(async (error) => {
    console.error(error);
    await prisma.$disconnect();
    process.exit(1);
  });
