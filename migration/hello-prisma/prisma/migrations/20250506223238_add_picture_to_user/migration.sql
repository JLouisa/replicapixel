-- AlterTable
ALTER TABLE "Users" ADD COLUMN     "picture" VARCHAR(255);

-- CreateIndex
CREATE INDEX "idx-fal_ai_request_id-pid" ON "Images"("fal_ai_request_id");
