// prisma/export_translations.ts
import { PrismaClient } from "./generated/prisma";
import fs from "fs";
import path from "path";

const prisma = new PrismaClient();

function bigIntToString(key: string, value: any) {
  return typeof value === "bigint" ? value.toString() : value;
}

async function exportTranslationsToJSON() {
  // Retrieve all Plans with their translations
  const plans = await prisma.plans.findMany({});

  // Retrieve all Packs with their translations
  const packs = await prisma.packs.findMany({});

  // Retrieve all Plans with their translations
  const plansTranslations = await prisma.plans_translations.findMany({});

  // Retrieve all Packs with their translations
  const packsTranslations = await prisma.packs_translations.findMany({});

  // Save to files with bigIntToString replacer
  fs.writeFileSync(path.resolve(__dirname, "plans.json"), JSON.stringify(plans, bigIntToString, 2));

  fs.writeFileSync(path.resolve(__dirname, "packs.json"), JSON.stringify(packs, bigIntToString, 2));

  fs.writeFileSync(
    path.resolve(__dirname, "plans_translations.json"),
    JSON.stringify(plansTranslations, bigIntToString, 2)
  );

  fs.writeFileSync(
    path.resolve(__dirname, "packs_translations.json"),
    JSON.stringify(packsTranslations, bigIntToString, 2)
  );

  console.log("Export finished.");
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
