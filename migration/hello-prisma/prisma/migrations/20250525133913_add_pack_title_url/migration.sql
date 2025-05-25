-- AlterTable
ALTER TABLE "Packs" ADD COLUMN     "popular" BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN     "title_url" VARCHAR(255) NOT NULL DEFAULT 'title-url',
ALTER COLUMN "num_inference_steps" SET DEFAULT 50;

-- CreateIndex
CREATE INDEX "idx-title-pid" ON "Packs"("title_url");
