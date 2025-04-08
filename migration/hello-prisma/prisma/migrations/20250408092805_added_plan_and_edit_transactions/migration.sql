/*
  Warnings:

  - You are about to drop the column `order_id` on the `Transactions` table. All the data in the column will be lost.
  - You are about to drop the column `plan` on the `Transactions` table. All the data in the column will be lost.
  - You are about to alter the column `currency` on the `Transactions` table. The data in that column could be lost. The data in that column will be cast from `VarChar(255)` to `VarChar(16)`.
  - Added the required column `plan_id` to the `Transactions` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Transactions" DROP COLUMN "order_id",
DROP COLUMN "plan",
ADD COLUMN     "plan_id" INTEGER NOT NULL,
ALTER COLUMN "currency" SET DATA TYPE VARCHAR(16);

-- CreateTable
CREATE TABLE "Plans" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "credit_amount" INTEGER NOT NULL,
    "model_amount" INTEGER NOT NULL,
    "price_cents" INTEGER NOT NULL,
    "stripe_price_id" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Plans_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Plans_pid_key" ON "Plans"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "Plans_name_key" ON "Plans"("name");

-- CreateIndex
CREATE UNIQUE INDEX "Plans_stripe_price_id_key" ON "Plans"("stripe_price_id");

-- CreateIndex
CREATE INDEX "idx-plan-pid" ON "Plans"("pid");

-- AddForeignKey
ALTER TABLE "Transactions" ADD CONSTRAINT "Transactions_plan_id_fkey" FOREIGN KEY ("plan_id") REFERENCES "Plans"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
