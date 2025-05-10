/*
  Warnings:

  - You are about to drop the column `starts` on the `Packs` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Packs" DROP COLUMN "starts",
ADD COLUMN     "stars" INTEGER NOT NULL DEFAULT 5;
