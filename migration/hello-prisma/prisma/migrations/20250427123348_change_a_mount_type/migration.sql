/*
  Warnings:

  - Added the required column `payment_amount` to the `Transactions` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Plans" ALTER COLUMN "price_cents" SET DATA TYPE BIGINT;

-- AlterTable
ALTER TABLE "Transactions" ADD COLUMN     "payment_amount" BIGINT NOT NULL;
