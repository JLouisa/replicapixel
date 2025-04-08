/*
  Warnings:

  - A unique constraint covering the columns `[plan_name]` on the table `Plans` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateTable
CREATE TABLE "handled_stripe_events" (
    "session_id" TEXT NOT NULL,
    "processed_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "handled_stripe_events_pkey" PRIMARY KEY ("session_id")
);

-- CreateTable
CREATE TABLE "handled_fal_events" (
    "request_id" TEXT NOT NULL,
    "processed_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "handled_fal_events_pkey" PRIMARY KEY ("request_id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Plans_plan_name_key" ON "Plans"("plan_name");
