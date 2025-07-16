/*
  Warnings:

  - A unique constraint covering the columns `[fal_ai_request_id]` on the table `Images` will be added. If there are existing duplicate values, this will fail.
  - A unique constraint covering the columns `[fal_ai_request_id]` on the table `Videos` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX "Images_fal_ai_request_id_key" ON "Images"("fal_ai_request_id");

-- CreateIndex
CREATE UNIQUE INDEX "Videos_fal_ai_request_id_key" ON "Videos"("fal_ai_request_id");
