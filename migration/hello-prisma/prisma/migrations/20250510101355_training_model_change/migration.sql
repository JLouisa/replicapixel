/*
  Warnings:

  - You are about to drop the column `image_url` on the `Packs` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Images" ALTER COLUMN "training_model_id" DROP NOT NULL;

-- AlterTable
ALTER TABLE "Packs" DROP COLUMN "image_url",
ADD COLUMN     "features" TEXT[] DEFAULT ARRAY[]::TEXT[],
ADD COLUMN     "images" TEXT[] DEFAULT ARRAY[]::TEXT[],
ADD COLUMN     "main_image" TEXT NOT NULL DEFAULT 'url',
ADD COLUMN     "starts" INTEGER NOT NULL DEFAULT 5,
ADD COLUMN     "used" INTEGER NOT NULL DEFAULT 0;
