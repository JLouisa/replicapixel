/*
  Warnings:

  - You are about to drop the column `amount` on the `Packs` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Packs" DROP COLUMN "amount",
ADD COLUMN     "num_images" INTEGER NOT NULL DEFAULT 0,
ADD COLUMN     "num_inference_steps" INTEGER NOT NULL DEFAULT 0;
