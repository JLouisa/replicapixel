/*
  Warnings:

  - A unique constraint covering the columns `[title_url]` on the table `Packs` will be added. If there are existing duplicate values, this will fail.

*/
-- AlterTable
ALTER TABLE "Packs" ALTER COLUMN "title_url" DROP DEFAULT;

-- CreateIndex
CREATE UNIQUE INDEX "Packs_title_url_key" ON "Packs"("title_url");
