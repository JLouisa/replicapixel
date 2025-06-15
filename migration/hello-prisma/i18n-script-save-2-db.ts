// prisma/import_translations.ts
import { PrismaClient } from "./generated/prisma";
import fs from "fs";
import path from "path";

const prisma = new PrismaClient();

async function importTranslationsFromJSON() {
  // Parse the files to get the data array
  const plans_json: any[] = JSON.parse(
    fs.readFileSync(path.resolve(__dirname, "plans_translated.json"), "utf-8")
  );

  const packs_json: any[] = JSON.parse(
    fs.readFileSync(path.resolve(__dirname, "packs_translated.json"), "utf-8")
  );

  // Sort by IDs to insert in a deterministic order
  plans_json.sort((a: number, b: number) => a.plan_id - b.plan_id);
  packs_json.sort((a: number, b: number) => a.pack_id - b.pack_id);

  // Insert into the database
  await prisma.plans_translations.createMany({ data: plans_json });
  console.log("Plans translations successfully imported.");

  await prisma.packs_translations.createMany({ data: packs_json });
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
