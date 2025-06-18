-- CreateIndex
CREATE INDEX "idx-fal_ai_request_id-training" ON "TrainingModels"("fal_ai_request_id");

-- RenameIndex
ALTER INDEX "idx-fal_ai_request_id-pid" RENAME TO "idx-fal_ai_request_id-image";
