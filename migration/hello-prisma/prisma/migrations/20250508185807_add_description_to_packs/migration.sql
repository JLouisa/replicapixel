/*
  Warnings:

  - You are about to drop the column `description` on the `Packs` table. All the data in the column will be lost.
  - Added the required column `full_description` to the `Packs` table without a default value. This is not possible if the table is not empty.
  - Added the required column `short_description` to the `Packs` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Packs" DROP COLUMN "description",
ADD COLUMN     "full_description" TEXT NOT NULL,
ADD COLUMN     "short_description" VARCHAR(255) NOT NULL;
