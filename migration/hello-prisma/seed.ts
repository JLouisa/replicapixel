// prisma/import_translations.ts
import { PrismaClient } from "./generated/prisma";
import fs from "fs";
import path from "path";

const prisma = new PrismaClient();

async function importTranslationsFromJSON() {
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

  plans_json.sort((a: any, b: any) => a.id - b.id);
  packs_json.sort((a: any, b: any) => a.id - b.id);
  plansTranslations.sort((a: any, b: any) => a.pack_id - b.pack_id);
  packsTranslations.sort((a: any, b: any) => a.pack_id - b.pack_id);

  await prisma.plans.createMany({ data: plans_json, skipDuplicates: true });
  console.log("Plans successfully imported.");

  await prisma.packs.createMany({ data: packs_json, skipDuplicates: true });
  console.log("Packs successfully imported.");

  await prisma.plans_translations.createMany({ data: plansTranslations, skipDuplicates: true });
  console.log("Plans translations successfully imported.");

  await prisma.packs_translations.createMany({ data: packsTranslations, skipDuplicates: true });
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
