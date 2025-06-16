// prisma/import_translations.ts
import { PrismaClient } from "./generated/prisma";
import fs from "fs";
import path from "path";

const prisma = new PrismaClient();

async function importTranslationsFromJSON() {
  // Parse the files to get the data array
  const plans_json: any[] = JSON.parse(
    fs.readFileSync(path.resolve(__dirname, "plans.json"), "utf-8")
  );

  const packs_json: any[] = JSON.parse(
    fs.readFileSync(path.resolve(__dirname, "packs.json"), "utf-8")
  );

  const plansTranslations: any[] = JSON.parse(
    fs.readFileSync(path.resolve(__dirname, "plans_translations.json"), "utf-8")
  );

  const packsTranslations: any[] = JSON.parse(
    fs.readFileSync(path.resolve(__dirname, "packs_translations.json"), "utf-8")
  );

  // Sort by IDs to insert in a deterministic order
  plans_json.sort((a: number, b: number) => a.id - b.id);
  packs_json.sort((a: number, b: number) => a.id - b.id);
  plansTranslations.sort((a: number, b: number) => a.pack_id - b.pack_id);
  packsTranslations.sort((a: number, b: number) => a.pack_id - b.pack_id);

  // Insert into the database
  await prisma.plans.createMany({ data: plans_json });
  console.log("Plans successfully imported.");
  await prisma.packs.createMany({ data: packs_json });
  console.log("packs successfully imported.");

  await prisma.plans_translations.createMany({ data: plansTranslations });
  console.log("Plans translations successfully imported.");

  await prisma.packs_translations.createMany({ data: packsTranslations });
  console.log("Packs translations successfully imported.");

  console.log("Import finished.");
}

importTranslationsFromJSON()
  .then(async () => {
    await prisma.$disconnect();
  })
  .catch(async (error) => {
    console.error(error);
    await prisma.$disconnect();
    process.exit(1);
  });
