/*
  Warnings:

  - Added the required column `pack_id` to the `Images` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Images" ADD COLUMN     "pack_id" INTEGER NOT NULL,
ALTER COLUMN "user_prompt" SET DATA TYPE TEXT,
ALTER COLUMN "sys_prompt" SET DATA TYPE TEXT,
ALTER COLUMN "image_url" SET DATA TYPE TEXT,
ALTER COLUMN "image_url_s3" SET DATA TYPE TEXT;

-- AlterTable
ALTER TABLE "Packs" ALTER COLUMN "description" SET DATA TYPE TEXT,
ALTER COLUMN "pack_prompts" SET DATA TYPE TEXT,
ALTER COLUMN "image_url" SET DATA TYPE TEXT;

-- AlterTable
ALTER TABLE "TrainingModels" ALTER COLUMN "thumbnail" SET DATA TYPE TEXT,
ALTER COLUMN "tensor_path" SET DATA TYPE TEXT;

-- AddForeignKey
ALTER TABLE "Images" ADD CONSTRAINT "Images_pack_id_fkey" FOREIGN KEY ("pack_id") REFERENCES "Packs"("id") ON DELETE CASCADE ON UPDATE CASCADE;
