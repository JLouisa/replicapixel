
Object.defineProperty(exports, "__esModule", { value: true });

const {
  Decimal,
  objectEnumValues,
  makeStrictEnum,
  Public,
  getRuntime,
  skip
} = require('./runtime/index-browser.js')


const Prisma = {}

exports.Prisma = Prisma
exports.$Enums = {}

/**
 * Prisma Client JS version: 6.5.0
 * Query Engine version: 173f8d54f8d52e692c7e27e72a88314ec7aeff60
 */
Prisma.prismaVersion = {
  client: "6.5.0",
  engine: "173f8d54f8d52e692c7e27e72a88314ec7aeff60"
}

Prisma.PrismaClientKnownRequestError = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`PrismaClientKnownRequestError is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)};
Prisma.PrismaClientUnknownRequestError = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`PrismaClientUnknownRequestError is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)}
Prisma.PrismaClientRustPanicError = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`PrismaClientRustPanicError is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)}
Prisma.PrismaClientInitializationError = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`PrismaClientInitializationError is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)}
Prisma.PrismaClientValidationError = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`PrismaClientValidationError is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)}
Prisma.Decimal = Decimal

/**
 * Re-export of sql-template-tag
 */
Prisma.sql = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`sqltag is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)}
Prisma.empty = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`empty is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)}
Prisma.join = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`join is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)}
Prisma.raw = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`raw is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)}
Prisma.validator = Public.validator

/**
* Extensions
*/
Prisma.getExtensionContext = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`Extensions.getExtensionContext is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)}
Prisma.defineExtension = () => {
  const runtimeName = getRuntime().prettyName;
  throw new Error(`Extensions.defineExtension is unable to run in this browser environment, or has been bundled for the browser (running in ${runtimeName}).
In case this error is unexpected for you, please report it in https://pris.ly/prisma-prisma-bug-report`,
)}

/**
 * Shorthand utilities for JSON filtering
 */
Prisma.DbNull = objectEnumValues.instances.DbNull
Prisma.JsonNull = objectEnumValues.instances.JsonNull
Prisma.AnyNull = objectEnumValues.instances.AnyNull

Prisma.NullTypes = {
  DbNull: objectEnumValues.classes.DbNull,
  JsonNull: objectEnumValues.classes.JsonNull,
  AnyNull: objectEnumValues.classes.AnyNull
}



/**
 * Enums
 */

exports.Prisma.TransactionIsolationLevel = makeStrictEnum({
  ReadUncommitted: 'ReadUncommitted',
  ReadCommitted: 'ReadCommitted',
  RepeatableRead: 'RepeatableRead',
  Serializable: 'Serializable'
});

exports.Prisma.UsersScalarFieldEnum = {
  id: 'id',
  pid: 'pid',
  email: 'email',
  password: 'password',
  api_key: 'api_key',
  name: 'name',
  reset_token: 'reset_token',
  reset_sent_at: 'reset_sent_at',
  email_verification_token: 'email_verification_token',
  email_verification_sent_at: 'email_verification_sent_at',
  email_verified_at: 'email_verified_at',
  magicLink_token: 'magicLink_token',
  magicLink_expiration: 'magicLink_expiration'
};

exports.Prisma.TrainingModelsScalarFieldEnum = {
  id: 'id',
  pid: 'pid',
  user_id: 'user_id',
  name: 'name',
  age: 'age',
  sex: 'sex',
  ethnicity: 'ethnicity',
  basedOn: 'basedOn',
  eye_color: 'eye_color',
  bald: 'bald',
  steps: 'steps',
  create_mask: 'create_mask',
  is_style: 'is_style',
  trigger_word: 'trigger_word',
  tensor_path: 'tensor_path',
  thumbnail: 'thumbnail',
  training_status: 'training_status',
  fal_output: 'fal_output',
  training_images: 'training_images',
  fal_ai_request_id: 'fal_ai_request_id',
  s3_key: 's3_key',
  is_verified: 'is_verified',
  deleted_at: 'deleted_at',
  created_at: 'created_at',
  updated_at: 'updated_at'
};

exports.Prisma.UserCreditsScalarFieldEnum = {
  id: 'id',
  pid: 'pid',
  user_id: 'user_id',
  credit_amount: 'credit_amount',
  model_amount: 'model_amount',
  created_at: 'created_at',
  updated_at: 'updated_at'
};

exports.Prisma.ImagesScalarFieldEnum = {
  id: 'id',
  pid: 'pid',
  user_id: 'user_id',
  training_model_id: 'training_model_id',
  pack_id: 'pack_id',
  user_prompt: 'user_prompt',
  sys_prompt: 'sys_prompt',
  alt: 'alt',
  num_inference_steps: 'num_inference_steps',
  content_type: 'content_type',
  status: 'status',
  image_size: 'image_size',
  fal_ai_request_id: 'fal_ai_request_id',
  width: 'width',
  height: 'height',
  image_url: 'image_url',
  image_url_s3: 'image_url_s3',
  is_favorite: 'is_favorite',
  deleted_at: 'deleted_at',
  created_at: 'created_at',
  updated_at: 'updated_at'
};

exports.Prisma.TransactionsScalarFieldEnum = {
  id: 'id',
  pid: 'pid',
  user_id: 'user_id',
  credit_amount: 'credit_amount',
  model_amount: 'model_amount',
  currency: 'currency',
  payment_id: 'payment_id',
  order_id: 'order_id',
  plan: 'plan',
  status: 'status',
  created_at: 'created_at',
  updated_at: 'updated_at'
};

exports.Prisma.PacksScalarFieldEnum = {
  id: 'id',
  pid: 'pid',
  name: 'name',
  description: 'description',
  pack_prompts: 'pack_prompts',
  credits: 'credits',
  amount: 'amount',
  image_url: 'image_url',
  created_at: 'created_at',
  updated_at: 'updated_at'
};

exports.Prisma.SortOrder = {
  asc: 'asc',
  desc: 'desc'
};

exports.Prisma.NullableJsonNullValueInput = {
  DbNull: Prisma.DbNull,
  JsonNull: Prisma.JsonNull
};

exports.Prisma.QueryMode = {
  default: 'default',
  insensitive: 'insensitive'
};

exports.Prisma.NullsOrder = {
  first: 'first',
  last: 'last'
};

exports.Prisma.JsonNullValueFilter = {
  DbNull: Prisma.DbNull,
  JsonNull: Prisma.JsonNull,
  AnyNull: Prisma.AnyNull
};
exports.Sex = exports.$Enums.Sex = {
  Male: 'Male',
  Female: 'Female'
};

exports.Ethnicity = exports.$Enums.Ethnicity = {
  White: 'White',
  Black: 'Black',
  Pacific: 'Pacific',
  Hispanic: 'Hispanic',
  Asian: 'Asian',
  SouthEastAsian: 'SouthEastAsian',
  SouthAsian: 'SouthAsian',
  MiddleEastern: 'MiddleEastern'
};

exports.BasedOn = exports.$Enums.BasedOn = {
  RealPerson: 'RealPerson',
  CreateInfluencerAI: 'CreateInfluencerAI'
};

exports.EyeColor = exports.$Enums.EyeColor = {
  Brown: 'Brown',
  Blue: 'Blue',
  Green: 'Green',
  Grey: 'Grey',
  Hazel: 'Hazel',
  Red: 'Red'
};

exports.Status = exports.$Enums.Status = {
  Pending: 'Pending',
  Processing: 'Processing',
  Training: 'Training',
  Completed: 'Completed',
  Failed: 'Failed',
  Cancelled: 'Cancelled'
};

exports.ImageFormat = exports.$Enums.ImageFormat = {
  Png: 'Png',
  Jpeg: 'Jpeg',
  Zip: 'Zip'
};

exports.ImageSize = exports.$Enums.ImageSize = {
  Square: 'Square',
  SquareHD: 'SquareHD',
  Portrait43: 'Portrait43',
  Portrait169: 'Portrait169',
  Landscape43: 'Landscape43',
  Landscape169: 'Landscape169'
};

exports.Prisma.ModelName = {
  Users: 'Users',
  TrainingModels: 'TrainingModels',
  UserCredits: 'UserCredits',
  Images: 'Images',
  Transactions: 'Transactions',
  Packs: 'Packs'
};

/**
 * This is a stub Prisma Client that will error at runtime if called.
 */
class PrismaClient {
  constructor() {
    return new Proxy(this, {
      get(target, prop) {
        let message
        const runtime = getRuntime()
        if (runtime.isEdge) {
          message = `PrismaClient is not configured to run in ${runtime.prettyName}. In order to run Prisma Client on edge runtime, either:
- Use Prisma Accelerate: https://pris.ly/d/accelerate
- Use Driver Adapters: https://pris.ly/d/driver-adapters
`;
        } else {
          message = 'PrismaClient is unable to run in this browser environment, or has been bundled for the browser (running in `' + runtime.prettyName + '`).'
        }
        
        message += `
If this is unexpected, please open an issue: https://pris.ly/prisma-prisma-bug-report`

        throw new Error(message)
      }
    })
  }
}

exports.PrismaClient = PrismaClient

Object.assign(exports, Prisma)
