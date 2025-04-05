
/**
 * Client
**/

import * as runtime from './runtime/library.js';
import $Types = runtime.Types // general types
import $Public = runtime.Types.Public
import $Utils = runtime.Types.Utils
import $Extensions = runtime.Types.Extensions
import $Result = runtime.Types.Result

export type PrismaPromise<T> = $Public.PrismaPromise<T>


/**
 * Model Users
 * 
 */
export type Users = $Result.DefaultSelection<Prisma.$UsersPayload>
/**
 * Model TrainingModels
 * 
 */
export type TrainingModels = $Result.DefaultSelection<Prisma.$TrainingModelsPayload>
/**
 * Model UserCredits
 * 
 */
export type UserCredits = $Result.DefaultSelection<Prisma.$UserCreditsPayload>
/**
 * Model Images
 * 
 */
export type Images = $Result.DefaultSelection<Prisma.$ImagesPayload>
/**
 * Model Transactions
 * 
 */
export type Transactions = $Result.DefaultSelection<Prisma.$TransactionsPayload>
/**
 * Model Packs
 * 
 */
export type Packs = $Result.DefaultSelection<Prisma.$PacksPayload>

/**
 * Enums
 */
export namespace $Enums {
  export const Status: {
  Pending: 'Pending',
  Processing: 'Processing',
  Training: 'Training',
  Completed: 'Completed',
  Failed: 'Failed',
  Cancelled: 'Cancelled'
};

export type Status = (typeof Status)[keyof typeof Status]


export const Sex: {
  Male: 'Male',
  Female: 'Female'
};

export type Sex = (typeof Sex)[keyof typeof Sex]


export const Ethnicity: {
  White: 'White',
  Black: 'Black',
  Pacific: 'Pacific',
  Hispanic: 'Hispanic',
  Asian: 'Asian',
  SouthEastAsian: 'SouthEastAsian',
  SouthAsian: 'SouthAsian',
  MiddleEastern: 'MiddleEastern'
};

export type Ethnicity = (typeof Ethnicity)[keyof typeof Ethnicity]


export const BasedOn: {
  RealPerson: 'RealPerson',
  CreateInfluencerAI: 'CreateInfluencerAI'
};

export type BasedOn = (typeof BasedOn)[keyof typeof BasedOn]


export const EyeColor: {
  Brown: 'Brown',
  Blue: 'Blue',
  Green: 'Green',
  Grey: 'Grey',
  Hazel: 'Hazel',
  Red: 'Red'
};

export type EyeColor = (typeof EyeColor)[keyof typeof EyeColor]


export const Emotion: {
  Neutral: 'Neutral',
  Anger: 'Anger',
  Disgust: 'Disgust',
  Fear: 'Fear',
  Happy: 'Happy',
  Sad: 'Sad',
  Surprise: 'Surprise'
};

export type Emotion = (typeof Emotion)[keyof typeof Emotion]


export const ImageSize: {
  Square: 'Square',
  SquareHD: 'SquareHD',
  Portrait43: 'Portrait43',
  Portrait169: 'Portrait169',
  Landscape43: 'Landscape43',
  Landscape169: 'Landscape169'
};

export type ImageSize = (typeof ImageSize)[keyof typeof ImageSize]


export const ImageFormat: {
  Png: 'Png',
  Jpeg: 'Jpeg',
  Zip: 'Zip'
};

export type ImageFormat = (typeof ImageFormat)[keyof typeof ImageFormat]

}

export type Status = $Enums.Status

export const Status: typeof $Enums.Status

export type Sex = $Enums.Sex

export const Sex: typeof $Enums.Sex

export type Ethnicity = $Enums.Ethnicity

export const Ethnicity: typeof $Enums.Ethnicity

export type BasedOn = $Enums.BasedOn

export const BasedOn: typeof $Enums.BasedOn

export type EyeColor = $Enums.EyeColor

export const EyeColor: typeof $Enums.EyeColor

export type Emotion = $Enums.Emotion

export const Emotion: typeof $Enums.Emotion

export type ImageSize = $Enums.ImageSize

export const ImageSize: typeof $Enums.ImageSize

export type ImageFormat = $Enums.ImageFormat

export const ImageFormat: typeof $Enums.ImageFormat

/**
 * ##  Prisma Client ʲˢ
 *
 * Type-safe database client for TypeScript & Node.js
 * @example
 * ```
 * const prisma = new PrismaClient()
 * // Fetch zero or more Users
 * const users = await prisma.users.findMany()
 * ```
 *
 *
 * Read more in our [docs](https://www.prisma.io/docs/reference/tools-and-interfaces/prisma-client).
 */
export class PrismaClient<
  ClientOptions extends Prisma.PrismaClientOptions = Prisma.PrismaClientOptions,
  U = 'log' extends keyof ClientOptions ? ClientOptions['log'] extends Array<Prisma.LogLevel | Prisma.LogDefinition> ? Prisma.GetEvents<ClientOptions['log']> : never : never,
  ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs
> {
  [K: symbol]: { types: Prisma.TypeMap<ExtArgs>['other'] }

    /**
   * ##  Prisma Client ʲˢ
   *
   * Type-safe database client for TypeScript & Node.js
   * @example
   * ```
   * const prisma = new PrismaClient()
   * // Fetch zero or more Users
   * const users = await prisma.users.findMany()
   * ```
   *
   *
   * Read more in our [docs](https://www.prisma.io/docs/reference/tools-and-interfaces/prisma-client).
   */

  constructor(optionsArg ?: Prisma.Subset<ClientOptions, Prisma.PrismaClientOptions>);
  $on<V extends U>(eventType: V, callback: (event: V extends 'query' ? Prisma.QueryEvent : Prisma.LogEvent) => void): PrismaClient;

  /**
   * Connect with the database
   */
  $connect(): $Utils.JsPromise<void>;

  /**
   * Disconnect from the database
   */
  $disconnect(): $Utils.JsPromise<void>;

  /**
   * Add a middleware
   * @deprecated since 4.16.0. For new code, prefer client extensions instead.
   * @see https://pris.ly/d/extensions
   */
  $use(cb: Prisma.Middleware): void

/**
   * Executes a prepared raw query and returns the number of affected rows.
   * @example
   * ```
   * const result = await prisma.$executeRaw`UPDATE User SET cool = ${true} WHERE email = ${'user@email.com'};`
   * ```
   *
   * Read more in our [docs](https://www.prisma.io/docs/reference/tools-and-interfaces/prisma-client/raw-database-access).
   */
  $executeRaw<T = unknown>(query: TemplateStringsArray | Prisma.Sql, ...values: any[]): Prisma.PrismaPromise<number>;

  /**
   * Executes a raw query and returns the number of affected rows.
   * Susceptible to SQL injections, see documentation.
   * @example
   * ```
   * const result = await prisma.$executeRawUnsafe('UPDATE User SET cool = $1 WHERE email = $2 ;', true, 'user@email.com')
   * ```
   *
   * Read more in our [docs](https://www.prisma.io/docs/reference/tools-and-interfaces/prisma-client/raw-database-access).
   */
  $executeRawUnsafe<T = unknown>(query: string, ...values: any[]): Prisma.PrismaPromise<number>;

  /**
   * Performs a prepared raw query and returns the `SELECT` data.
   * @example
   * ```
   * const result = await prisma.$queryRaw`SELECT * FROM User WHERE id = ${1} OR email = ${'user@email.com'};`
   * ```
   *
   * Read more in our [docs](https://www.prisma.io/docs/reference/tools-and-interfaces/prisma-client/raw-database-access).
   */
  $queryRaw<T = unknown>(query: TemplateStringsArray | Prisma.Sql, ...values: any[]): Prisma.PrismaPromise<T>;

  /**
   * Performs a raw query and returns the `SELECT` data.
   * Susceptible to SQL injections, see documentation.
   * @example
   * ```
   * const result = await prisma.$queryRawUnsafe('SELECT * FROM User WHERE id = $1 OR email = $2;', 1, 'user@email.com')
   * ```
   *
   * Read more in our [docs](https://www.prisma.io/docs/reference/tools-and-interfaces/prisma-client/raw-database-access).
   */
  $queryRawUnsafe<T = unknown>(query: string, ...values: any[]): Prisma.PrismaPromise<T>;


  /**
   * Allows the running of a sequence of read/write operations that are guaranteed to either succeed or fail as a whole.
   * @example
   * ```
   * const [george, bob, alice] = await prisma.$transaction([
   *   prisma.user.create({ data: { name: 'George' } }),
   *   prisma.user.create({ data: { name: 'Bob' } }),
   *   prisma.user.create({ data: { name: 'Alice' } }),
   * ])
   * ```
   * 
   * Read more in our [docs](https://www.prisma.io/docs/concepts/components/prisma-client/transactions).
   */
  $transaction<P extends Prisma.PrismaPromise<any>[]>(arg: [...P], options?: { isolationLevel?: Prisma.TransactionIsolationLevel }): $Utils.JsPromise<runtime.Types.Utils.UnwrapTuple<P>>

  $transaction<R>(fn: (prisma: Omit<PrismaClient, runtime.ITXClientDenyList>) => $Utils.JsPromise<R>, options?: { maxWait?: number, timeout?: number, isolationLevel?: Prisma.TransactionIsolationLevel }): $Utils.JsPromise<R>


  $extends: $Extensions.ExtendsHook<"extends", Prisma.TypeMapCb<ClientOptions>, ExtArgs, $Utils.Call<Prisma.TypeMapCb<ClientOptions>, {
    extArgs: ExtArgs
  }>>

      /**
   * `prisma.users`: Exposes CRUD operations for the **Users** model.
    * Example usage:
    * ```ts
    * // Fetch zero or more Users
    * const users = await prisma.users.findMany()
    * ```
    */
  get users(): Prisma.UsersDelegate<ExtArgs, ClientOptions>;

  /**
   * `prisma.trainingModels`: Exposes CRUD operations for the **TrainingModels** model.
    * Example usage:
    * ```ts
    * // Fetch zero or more TrainingModels
    * const trainingModels = await prisma.trainingModels.findMany()
    * ```
    */
  get trainingModels(): Prisma.TrainingModelsDelegate<ExtArgs, ClientOptions>;

  /**
   * `prisma.userCredits`: Exposes CRUD operations for the **UserCredits** model.
    * Example usage:
    * ```ts
    * // Fetch zero or more UserCredits
    * const userCredits = await prisma.userCredits.findMany()
    * ```
    */
  get userCredits(): Prisma.UserCreditsDelegate<ExtArgs, ClientOptions>;

  /**
   * `prisma.images`: Exposes CRUD operations for the **Images** model.
    * Example usage:
    * ```ts
    * // Fetch zero or more Images
    * const images = await prisma.images.findMany()
    * ```
    */
  get images(): Prisma.ImagesDelegate<ExtArgs, ClientOptions>;

  /**
   * `prisma.transactions`: Exposes CRUD operations for the **Transactions** model.
    * Example usage:
    * ```ts
    * // Fetch zero or more Transactions
    * const transactions = await prisma.transactions.findMany()
    * ```
    */
  get transactions(): Prisma.TransactionsDelegate<ExtArgs, ClientOptions>;

  /**
   * `prisma.packs`: Exposes CRUD operations for the **Packs** model.
    * Example usage:
    * ```ts
    * // Fetch zero or more Packs
    * const packs = await prisma.packs.findMany()
    * ```
    */
  get packs(): Prisma.PacksDelegate<ExtArgs, ClientOptions>;
}

export namespace Prisma {
  export import DMMF = runtime.DMMF

  export type PrismaPromise<T> = $Public.PrismaPromise<T>

  /**
   * Validator
   */
  export import validator = runtime.Public.validator

  /**
   * Prisma Errors
   */
  export import PrismaClientKnownRequestError = runtime.PrismaClientKnownRequestError
  export import PrismaClientUnknownRequestError = runtime.PrismaClientUnknownRequestError
  export import PrismaClientRustPanicError = runtime.PrismaClientRustPanicError
  export import PrismaClientInitializationError = runtime.PrismaClientInitializationError
  export import PrismaClientValidationError = runtime.PrismaClientValidationError

  /**
   * Re-export of sql-template-tag
   */
  export import sql = runtime.sqltag
  export import empty = runtime.empty
  export import join = runtime.join
  export import raw = runtime.raw
  export import Sql = runtime.Sql



  /**
   * Decimal.js
   */
  export import Decimal = runtime.Decimal

  export type DecimalJsLike = runtime.DecimalJsLike

  /**
   * Metrics
   */
  export type Metrics = runtime.Metrics
  export type Metric<T> = runtime.Metric<T>
  export type MetricHistogram = runtime.MetricHistogram
  export type MetricHistogramBucket = runtime.MetricHistogramBucket

  /**
  * Extensions
  */
  export import Extension = $Extensions.UserArgs
  export import getExtensionContext = runtime.Extensions.getExtensionContext
  export import Args = $Public.Args
  export import Payload = $Public.Payload
  export import Result = $Public.Result
  export import Exact = $Public.Exact

  /**
   * Prisma Client JS version: 6.5.0
   * Query Engine version: 173f8d54f8d52e692c7e27e72a88314ec7aeff60
   */
  export type PrismaVersion = {
    client: string
  }

  export const prismaVersion: PrismaVersion

  /**
   * Utility Types
   */


  export import JsonObject = runtime.JsonObject
  export import JsonArray = runtime.JsonArray
  export import JsonValue = runtime.JsonValue
  export import InputJsonObject = runtime.InputJsonObject
  export import InputJsonArray = runtime.InputJsonArray
  export import InputJsonValue = runtime.InputJsonValue

  /**
   * Types of the values used to represent different kinds of `null` values when working with JSON fields.
   *
   * @see https://www.prisma.io/docs/concepts/components/prisma-client/working-with-fields/working-with-json-fields#filtering-on-a-json-field
   */
  namespace NullTypes {
    /**
    * Type of `Prisma.DbNull`.
    *
    * You cannot use other instances of this class. Please use the `Prisma.DbNull` value.
    *
    * @see https://www.prisma.io/docs/concepts/components/prisma-client/working-with-fields/working-with-json-fields#filtering-on-a-json-field
    */
    class DbNull {
      private DbNull: never
      private constructor()
    }

    /**
    * Type of `Prisma.JsonNull`.
    *
    * You cannot use other instances of this class. Please use the `Prisma.JsonNull` value.
    *
    * @see https://www.prisma.io/docs/concepts/components/prisma-client/working-with-fields/working-with-json-fields#filtering-on-a-json-field
    */
    class JsonNull {
      private JsonNull: never
      private constructor()
    }

    /**
    * Type of `Prisma.AnyNull`.
    *
    * You cannot use other instances of this class. Please use the `Prisma.AnyNull` value.
    *
    * @see https://www.prisma.io/docs/concepts/components/prisma-client/working-with-fields/working-with-json-fields#filtering-on-a-json-field
    */
    class AnyNull {
      private AnyNull: never
      private constructor()
    }
  }

  /**
   * Helper for filtering JSON entries that have `null` on the database (empty on the db)
   *
   * @see https://www.prisma.io/docs/concepts/components/prisma-client/working-with-fields/working-with-json-fields#filtering-on-a-json-field
   */
  export const DbNull: NullTypes.DbNull

  /**
   * Helper for filtering JSON entries that have JSON `null` values (not empty on the db)
   *
   * @see https://www.prisma.io/docs/concepts/components/prisma-client/working-with-fields/working-with-json-fields#filtering-on-a-json-field
   */
  export const JsonNull: NullTypes.JsonNull

  /**
   * Helper for filtering JSON entries that are `Prisma.DbNull` or `Prisma.JsonNull`
   *
   * @see https://www.prisma.io/docs/concepts/components/prisma-client/working-with-fields/working-with-json-fields#filtering-on-a-json-field
   */
  export const AnyNull: NullTypes.AnyNull

  type SelectAndInclude = {
    select: any
    include: any
  }

  type SelectAndOmit = {
    select: any
    omit: any
  }

  /**
   * Get the type of the value, that the Promise holds.
   */
  export type PromiseType<T extends PromiseLike<any>> = T extends PromiseLike<infer U> ? U : T;

  /**
   * Get the return type of a function which returns a Promise.
   */
  export type PromiseReturnType<T extends (...args: any) => $Utils.JsPromise<any>> = PromiseType<ReturnType<T>>

  /**
   * From T, pick a set of properties whose keys are in the union K
   */
  type Prisma__Pick<T, K extends keyof T> = {
      [P in K]: T[P];
  };


  export type Enumerable<T> = T | Array<T>;

  export type RequiredKeys<T> = {
    [K in keyof T]-?: {} extends Prisma__Pick<T, K> ? never : K
  }[keyof T]

  export type TruthyKeys<T> = keyof {
    [K in keyof T as T[K] extends false | undefined | null ? never : K]: K
  }

  export type TrueKeys<T> = TruthyKeys<Prisma__Pick<T, RequiredKeys<T>>>

  /**
   * Subset
   * @desc From `T` pick properties that exist in `U`. Simple version of Intersection
   */
  export type Subset<T, U> = {
    [key in keyof T]: key extends keyof U ? T[key] : never;
  };

  /**
   * SelectSubset
   * @desc From `T` pick properties that exist in `U`. Simple version of Intersection.
   * Additionally, it validates, if both select and include are present. If the case, it errors.
   */
  export type SelectSubset<T, U> = {
    [key in keyof T]: key extends keyof U ? T[key] : never
  } &
    (T extends SelectAndInclude
      ? 'Please either choose `select` or `include`.'
      : T extends SelectAndOmit
        ? 'Please either choose `select` or `omit`.'
        : {})

  /**
   * Subset + Intersection
   * @desc From `T` pick properties that exist in `U` and intersect `K`
   */
  export type SubsetIntersection<T, U, K> = {
    [key in keyof T]: key extends keyof U ? T[key] : never
  } &
    K

  type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };

  /**
   * XOR is needed to have a real mutually exclusive union type
   * https://stackoverflow.com/questions/42123407/does-typescript-support-mutually-exclusive-types
   */
  type XOR<T, U> =
    T extends object ?
    U extends object ?
      (Without<T, U> & U) | (Without<U, T> & T)
    : U : T


  /**
   * Is T a Record?
   */
  type IsObject<T extends any> = T extends Array<any>
  ? False
  : T extends Date
  ? False
  : T extends Uint8Array
  ? False
  : T extends BigInt
  ? False
  : T extends object
  ? True
  : False


  /**
   * If it's T[], return T
   */
  export type UnEnumerate<T extends unknown> = T extends Array<infer U> ? U : T

  /**
   * From ts-toolbelt
   */

  type __Either<O extends object, K extends Key> = Omit<O, K> &
    {
      // Merge all but K
      [P in K]: Prisma__Pick<O, P & keyof O> // With K possibilities
    }[K]

  type EitherStrict<O extends object, K extends Key> = Strict<__Either<O, K>>

  type EitherLoose<O extends object, K extends Key> = ComputeRaw<__Either<O, K>>

  type _Either<
    O extends object,
    K extends Key,
    strict extends Boolean
  > = {
    1: EitherStrict<O, K>
    0: EitherLoose<O, K>
  }[strict]

  type Either<
    O extends object,
    K extends Key,
    strict extends Boolean = 1
  > = O extends unknown ? _Either<O, K, strict> : never

  export type Union = any

  type PatchUndefined<O extends object, O1 extends object> = {
    [K in keyof O]: O[K] extends undefined ? At<O1, K> : O[K]
  } & {}

  /** Helper Types for "Merge" **/
  export type IntersectOf<U extends Union> = (
    U extends unknown ? (k: U) => void : never
  ) extends (k: infer I) => void
    ? I
    : never

  export type Overwrite<O extends object, O1 extends object> = {
      [K in keyof O]: K extends keyof O1 ? O1[K] : O[K];
  } & {};

  type _Merge<U extends object> = IntersectOf<Overwrite<U, {
      [K in keyof U]-?: At<U, K>;
  }>>;

  type Key = string | number | symbol;
  type AtBasic<O extends object, K extends Key> = K extends keyof O ? O[K] : never;
  type AtStrict<O extends object, K extends Key> = O[K & keyof O];
  type AtLoose<O extends object, K extends Key> = O extends unknown ? AtStrict<O, K> : never;
  export type At<O extends object, K extends Key, strict extends Boolean = 1> = {
      1: AtStrict<O, K>;
      0: AtLoose<O, K>;
  }[strict];

  export type ComputeRaw<A extends any> = A extends Function ? A : {
    [K in keyof A]: A[K];
  } & {};

  export type OptionalFlat<O> = {
    [K in keyof O]?: O[K];
  } & {};

  type _Record<K extends keyof any, T> = {
    [P in K]: T;
  };

  // cause typescript not to expand types and preserve names
  type NoExpand<T> = T extends unknown ? T : never;

  // this type assumes the passed object is entirely optional
  type AtLeast<O extends object, K extends string> = NoExpand<
    O extends unknown
    ? | (K extends keyof O ? { [P in K]: O[P] } & O : O)
      | {[P in keyof O as P extends K ? P : never]-?: O[P]} & O
    : never>;

  type _Strict<U, _U = U> = U extends unknown ? U & OptionalFlat<_Record<Exclude<Keys<_U>, keyof U>, never>> : never;

  export type Strict<U extends object> = ComputeRaw<_Strict<U>>;
  /** End Helper Types for "Merge" **/

  export type Merge<U extends object> = ComputeRaw<_Merge<Strict<U>>>;

  /**
  A [[Boolean]]
  */
  export type Boolean = True | False

  // /**
  // 1
  // */
  export type True = 1

  /**
  0
  */
  export type False = 0

  export type Not<B extends Boolean> = {
    0: 1
    1: 0
  }[B]

  export type Extends<A1 extends any, A2 extends any> = [A1] extends [never]
    ? 0 // anything `never` is false
    : A1 extends A2
    ? 1
    : 0

  export type Has<U extends Union, U1 extends Union> = Not<
    Extends<Exclude<U1, U>, U1>
  >

  export type Or<B1 extends Boolean, B2 extends Boolean> = {
    0: {
      0: 0
      1: 1
    }
    1: {
      0: 1
      1: 1
    }
  }[B1][B2]

  export type Keys<U extends Union> = U extends unknown ? keyof U : never

  type Cast<A, B> = A extends B ? A : B;

  export const type: unique symbol;



  /**
   * Used by group by
   */

  export type GetScalarType<T, O> = O extends object ? {
    [P in keyof T]: P extends keyof O
      ? O[P]
      : never
  } : never

  type FieldPaths<
    T,
    U = Omit<T, '_avg' | '_sum' | '_count' | '_min' | '_max'>
  > = IsObject<T> extends True ? U : T

  type GetHavingFields<T> = {
    [K in keyof T]: Or<
      Or<Extends<'OR', K>, Extends<'AND', K>>,
      Extends<'NOT', K>
    > extends True
      ? // infer is only needed to not hit TS limit
        // based on the brilliant idea of Pierre-Antoine Mills
        // https://github.com/microsoft/TypeScript/issues/30188#issuecomment-478938437
        T[K] extends infer TK
        ? GetHavingFields<UnEnumerate<TK> extends object ? Merge<UnEnumerate<TK>> : never>
        : never
      : {} extends FieldPaths<T[K]>
      ? never
      : K
  }[keyof T]

  /**
   * Convert tuple to union
   */
  type _TupleToUnion<T> = T extends (infer E)[] ? E : never
  type TupleToUnion<K extends readonly any[]> = _TupleToUnion<K>
  type MaybeTupleToUnion<T> = T extends any[] ? TupleToUnion<T> : T

  /**
   * Like `Pick`, but additionally can also accept an array of keys
   */
  type PickEnumerable<T, K extends Enumerable<keyof T> | keyof T> = Prisma__Pick<T, MaybeTupleToUnion<K>>

  /**
   * Exclude all keys with underscores
   */
  type ExcludeUnderscoreKeys<T extends string> = T extends `_${string}` ? never : T


  export type FieldRef<Model, FieldType> = runtime.FieldRef<Model, FieldType>

  type FieldRefInputType<Model, FieldType> = Model extends never ? never : FieldRef<Model, FieldType>


  export const ModelName: {
    Users: 'Users',
    TrainingModels: 'TrainingModels',
    UserCredits: 'UserCredits',
    Images: 'Images',
    Transactions: 'Transactions',
    Packs: 'Packs'
  };

  export type ModelName = (typeof ModelName)[keyof typeof ModelName]


  export type Datasources = {
    db?: Datasource
  }

  interface TypeMapCb<ClientOptions = {}> extends $Utils.Fn<{extArgs: $Extensions.InternalArgs }, $Utils.Record<string, any>> {
    returns: Prisma.TypeMap<this['params']['extArgs'], ClientOptions extends { omit: infer OmitOptions } ? OmitOptions : {}>
  }

  export type TypeMap<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> = {
    globalOmitOptions: {
      omit: GlobalOmitOptions
    }
    meta: {
      modelProps: "users" | "trainingModels" | "userCredits" | "images" | "transactions" | "packs"
      txIsolationLevel: Prisma.TransactionIsolationLevel
    }
    model: {
      Users: {
        payload: Prisma.$UsersPayload<ExtArgs>
        fields: Prisma.UsersFieldRefs
        operations: {
          findUnique: {
            args: Prisma.UsersFindUniqueArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload> | null
          }
          findUniqueOrThrow: {
            args: Prisma.UsersFindUniqueOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload>
          }
          findFirst: {
            args: Prisma.UsersFindFirstArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload> | null
          }
          findFirstOrThrow: {
            args: Prisma.UsersFindFirstOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload>
          }
          findMany: {
            args: Prisma.UsersFindManyArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload>[]
          }
          create: {
            args: Prisma.UsersCreateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload>
          }
          createMany: {
            args: Prisma.UsersCreateManyArgs<ExtArgs>
            result: BatchPayload
          }
          createManyAndReturn: {
            args: Prisma.UsersCreateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload>[]
          }
          delete: {
            args: Prisma.UsersDeleteArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload>
          }
          update: {
            args: Prisma.UsersUpdateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload>
          }
          deleteMany: {
            args: Prisma.UsersDeleteManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateMany: {
            args: Prisma.UsersUpdateManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateManyAndReturn: {
            args: Prisma.UsersUpdateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload>[]
          }
          upsert: {
            args: Prisma.UsersUpsertArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UsersPayload>
          }
          aggregate: {
            args: Prisma.UsersAggregateArgs<ExtArgs>
            result: $Utils.Optional<AggregateUsers>
          }
          groupBy: {
            args: Prisma.UsersGroupByArgs<ExtArgs>
            result: $Utils.Optional<UsersGroupByOutputType>[]
          }
          count: {
            args: Prisma.UsersCountArgs<ExtArgs>
            result: $Utils.Optional<UsersCountAggregateOutputType> | number
          }
        }
      }
      TrainingModels: {
        payload: Prisma.$TrainingModelsPayload<ExtArgs>
        fields: Prisma.TrainingModelsFieldRefs
        operations: {
          findUnique: {
            args: Prisma.TrainingModelsFindUniqueArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload> | null
          }
          findUniqueOrThrow: {
            args: Prisma.TrainingModelsFindUniqueOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload>
          }
          findFirst: {
            args: Prisma.TrainingModelsFindFirstArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload> | null
          }
          findFirstOrThrow: {
            args: Prisma.TrainingModelsFindFirstOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload>
          }
          findMany: {
            args: Prisma.TrainingModelsFindManyArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload>[]
          }
          create: {
            args: Prisma.TrainingModelsCreateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload>
          }
          createMany: {
            args: Prisma.TrainingModelsCreateManyArgs<ExtArgs>
            result: BatchPayload
          }
          createManyAndReturn: {
            args: Prisma.TrainingModelsCreateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload>[]
          }
          delete: {
            args: Prisma.TrainingModelsDeleteArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload>
          }
          update: {
            args: Prisma.TrainingModelsUpdateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload>
          }
          deleteMany: {
            args: Prisma.TrainingModelsDeleteManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateMany: {
            args: Prisma.TrainingModelsUpdateManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateManyAndReturn: {
            args: Prisma.TrainingModelsUpdateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload>[]
          }
          upsert: {
            args: Prisma.TrainingModelsUpsertArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TrainingModelsPayload>
          }
          aggregate: {
            args: Prisma.TrainingModelsAggregateArgs<ExtArgs>
            result: $Utils.Optional<AggregateTrainingModels>
          }
          groupBy: {
            args: Prisma.TrainingModelsGroupByArgs<ExtArgs>
            result: $Utils.Optional<TrainingModelsGroupByOutputType>[]
          }
          count: {
            args: Prisma.TrainingModelsCountArgs<ExtArgs>
            result: $Utils.Optional<TrainingModelsCountAggregateOutputType> | number
          }
        }
      }
      UserCredits: {
        payload: Prisma.$UserCreditsPayload<ExtArgs>
        fields: Prisma.UserCreditsFieldRefs
        operations: {
          findUnique: {
            args: Prisma.UserCreditsFindUniqueArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload> | null
          }
          findUniqueOrThrow: {
            args: Prisma.UserCreditsFindUniqueOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload>
          }
          findFirst: {
            args: Prisma.UserCreditsFindFirstArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload> | null
          }
          findFirstOrThrow: {
            args: Prisma.UserCreditsFindFirstOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload>
          }
          findMany: {
            args: Prisma.UserCreditsFindManyArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload>[]
          }
          create: {
            args: Prisma.UserCreditsCreateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload>
          }
          createMany: {
            args: Prisma.UserCreditsCreateManyArgs<ExtArgs>
            result: BatchPayload
          }
          createManyAndReturn: {
            args: Prisma.UserCreditsCreateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload>[]
          }
          delete: {
            args: Prisma.UserCreditsDeleteArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload>
          }
          update: {
            args: Prisma.UserCreditsUpdateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload>
          }
          deleteMany: {
            args: Prisma.UserCreditsDeleteManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateMany: {
            args: Prisma.UserCreditsUpdateManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateManyAndReturn: {
            args: Prisma.UserCreditsUpdateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload>[]
          }
          upsert: {
            args: Prisma.UserCreditsUpsertArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$UserCreditsPayload>
          }
          aggregate: {
            args: Prisma.UserCreditsAggregateArgs<ExtArgs>
            result: $Utils.Optional<AggregateUserCredits>
          }
          groupBy: {
            args: Prisma.UserCreditsGroupByArgs<ExtArgs>
            result: $Utils.Optional<UserCreditsGroupByOutputType>[]
          }
          count: {
            args: Prisma.UserCreditsCountArgs<ExtArgs>
            result: $Utils.Optional<UserCreditsCountAggregateOutputType> | number
          }
        }
      }
      Images: {
        payload: Prisma.$ImagesPayload<ExtArgs>
        fields: Prisma.ImagesFieldRefs
        operations: {
          findUnique: {
            args: Prisma.ImagesFindUniqueArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload> | null
          }
          findUniqueOrThrow: {
            args: Prisma.ImagesFindUniqueOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload>
          }
          findFirst: {
            args: Prisma.ImagesFindFirstArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload> | null
          }
          findFirstOrThrow: {
            args: Prisma.ImagesFindFirstOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload>
          }
          findMany: {
            args: Prisma.ImagesFindManyArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload>[]
          }
          create: {
            args: Prisma.ImagesCreateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload>
          }
          createMany: {
            args: Prisma.ImagesCreateManyArgs<ExtArgs>
            result: BatchPayload
          }
          createManyAndReturn: {
            args: Prisma.ImagesCreateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload>[]
          }
          delete: {
            args: Prisma.ImagesDeleteArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload>
          }
          update: {
            args: Prisma.ImagesUpdateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload>
          }
          deleteMany: {
            args: Prisma.ImagesDeleteManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateMany: {
            args: Prisma.ImagesUpdateManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateManyAndReturn: {
            args: Prisma.ImagesUpdateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload>[]
          }
          upsert: {
            args: Prisma.ImagesUpsertArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$ImagesPayload>
          }
          aggregate: {
            args: Prisma.ImagesAggregateArgs<ExtArgs>
            result: $Utils.Optional<AggregateImages>
          }
          groupBy: {
            args: Prisma.ImagesGroupByArgs<ExtArgs>
            result: $Utils.Optional<ImagesGroupByOutputType>[]
          }
          count: {
            args: Prisma.ImagesCountArgs<ExtArgs>
            result: $Utils.Optional<ImagesCountAggregateOutputType> | number
          }
        }
      }
      Transactions: {
        payload: Prisma.$TransactionsPayload<ExtArgs>
        fields: Prisma.TransactionsFieldRefs
        operations: {
          findUnique: {
            args: Prisma.TransactionsFindUniqueArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload> | null
          }
          findUniqueOrThrow: {
            args: Prisma.TransactionsFindUniqueOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload>
          }
          findFirst: {
            args: Prisma.TransactionsFindFirstArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload> | null
          }
          findFirstOrThrow: {
            args: Prisma.TransactionsFindFirstOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload>
          }
          findMany: {
            args: Prisma.TransactionsFindManyArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload>[]
          }
          create: {
            args: Prisma.TransactionsCreateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload>
          }
          createMany: {
            args: Prisma.TransactionsCreateManyArgs<ExtArgs>
            result: BatchPayload
          }
          createManyAndReturn: {
            args: Prisma.TransactionsCreateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload>[]
          }
          delete: {
            args: Prisma.TransactionsDeleteArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload>
          }
          update: {
            args: Prisma.TransactionsUpdateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload>
          }
          deleteMany: {
            args: Prisma.TransactionsDeleteManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateMany: {
            args: Prisma.TransactionsUpdateManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateManyAndReturn: {
            args: Prisma.TransactionsUpdateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload>[]
          }
          upsert: {
            args: Prisma.TransactionsUpsertArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$TransactionsPayload>
          }
          aggregate: {
            args: Prisma.TransactionsAggregateArgs<ExtArgs>
            result: $Utils.Optional<AggregateTransactions>
          }
          groupBy: {
            args: Prisma.TransactionsGroupByArgs<ExtArgs>
            result: $Utils.Optional<TransactionsGroupByOutputType>[]
          }
          count: {
            args: Prisma.TransactionsCountArgs<ExtArgs>
            result: $Utils.Optional<TransactionsCountAggregateOutputType> | number
          }
        }
      }
      Packs: {
        payload: Prisma.$PacksPayload<ExtArgs>
        fields: Prisma.PacksFieldRefs
        operations: {
          findUnique: {
            args: Prisma.PacksFindUniqueArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload> | null
          }
          findUniqueOrThrow: {
            args: Prisma.PacksFindUniqueOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload>
          }
          findFirst: {
            args: Prisma.PacksFindFirstArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload> | null
          }
          findFirstOrThrow: {
            args: Prisma.PacksFindFirstOrThrowArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload>
          }
          findMany: {
            args: Prisma.PacksFindManyArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload>[]
          }
          create: {
            args: Prisma.PacksCreateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload>
          }
          createMany: {
            args: Prisma.PacksCreateManyArgs<ExtArgs>
            result: BatchPayload
          }
          createManyAndReturn: {
            args: Prisma.PacksCreateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload>[]
          }
          delete: {
            args: Prisma.PacksDeleteArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload>
          }
          update: {
            args: Prisma.PacksUpdateArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload>
          }
          deleteMany: {
            args: Prisma.PacksDeleteManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateMany: {
            args: Prisma.PacksUpdateManyArgs<ExtArgs>
            result: BatchPayload
          }
          updateManyAndReturn: {
            args: Prisma.PacksUpdateManyAndReturnArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload>[]
          }
          upsert: {
            args: Prisma.PacksUpsertArgs<ExtArgs>
            result: $Utils.PayloadToResult<Prisma.$PacksPayload>
          }
          aggregate: {
            args: Prisma.PacksAggregateArgs<ExtArgs>
            result: $Utils.Optional<AggregatePacks>
          }
          groupBy: {
            args: Prisma.PacksGroupByArgs<ExtArgs>
            result: $Utils.Optional<PacksGroupByOutputType>[]
          }
          count: {
            args: Prisma.PacksCountArgs<ExtArgs>
            result: $Utils.Optional<PacksCountAggregateOutputType> | number
          }
        }
      }
    }
  } & {
    other: {
      payload: any
      operations: {
        $executeRaw: {
          args: [query: TemplateStringsArray | Prisma.Sql, ...values: any[]],
          result: any
        }
        $executeRawUnsafe: {
          args: [query: string, ...values: any[]],
          result: any
        }
        $queryRaw: {
          args: [query: TemplateStringsArray | Prisma.Sql, ...values: any[]],
          result: any
        }
        $queryRawUnsafe: {
          args: [query: string, ...values: any[]],
          result: any
        }
      }
    }
  }
  export const defineExtension: $Extensions.ExtendsHook<"define", Prisma.TypeMapCb, $Extensions.DefaultArgs>
  export type DefaultPrismaClient = PrismaClient
  export type ErrorFormat = 'pretty' | 'colorless' | 'minimal'
  export interface PrismaClientOptions {
    /**
     * Overwrites the datasource url from your schema.prisma file
     */
    datasources?: Datasources
    /**
     * Overwrites the datasource url from your schema.prisma file
     */
    datasourceUrl?: string
    /**
     * @default "colorless"
     */
    errorFormat?: ErrorFormat
    /**
     * @example
     * ```
     * // Defaults to stdout
     * log: ['query', 'info', 'warn', 'error']
     * 
     * // Emit as events
     * log: [
     *   { emit: 'stdout', level: 'query' },
     *   { emit: 'stdout', level: 'info' },
     *   { emit: 'stdout', level: 'warn' }
     *   { emit: 'stdout', level: 'error' }
     * ]
     * ```
     * Read more in our [docs](https://www.prisma.io/docs/reference/tools-and-interfaces/prisma-client/logging#the-log-option).
     */
    log?: (LogLevel | LogDefinition)[]
    /**
     * The default values for transactionOptions
     * maxWait ?= 2000
     * timeout ?= 5000
     */
    transactionOptions?: {
      maxWait?: number
      timeout?: number
      isolationLevel?: Prisma.TransactionIsolationLevel
    }
    /**
     * Global configuration for omitting model fields by default.
     * 
     * @example
     * ```
     * const prisma = new PrismaClient({
     *   omit: {
     *     user: {
     *       password: true
     *     }
     *   }
     * })
     * ```
     */
    omit?: Prisma.GlobalOmitConfig
  }
  export type GlobalOmitConfig = {
    users?: UsersOmit
    trainingModels?: TrainingModelsOmit
    userCredits?: UserCreditsOmit
    images?: ImagesOmit
    transactions?: TransactionsOmit
    packs?: PacksOmit
  }

  /* Types for Logging */
  export type LogLevel = 'info' | 'query' | 'warn' | 'error'
  export type LogDefinition = {
    level: LogLevel
    emit: 'stdout' | 'event'
  }

  export type GetLogType<T extends LogLevel | LogDefinition> = T extends LogDefinition ? T['emit'] extends 'event' ? T['level'] : never : never
  export type GetEvents<T extends any> = T extends Array<LogLevel | LogDefinition> ?
    GetLogType<T[0]> | GetLogType<T[1]> | GetLogType<T[2]> | GetLogType<T[3]>
    : never

  export type QueryEvent = {
    timestamp: Date
    query: string
    params: string
    duration: number
    target: string
  }

  export type LogEvent = {
    timestamp: Date
    message: string
    target: string
  }
  /* End Types for Logging */


  export type PrismaAction =
    | 'findUnique'
    | 'findUniqueOrThrow'
    | 'findMany'
    | 'findFirst'
    | 'findFirstOrThrow'
    | 'create'
    | 'createMany'
    | 'createManyAndReturn'
    | 'update'
    | 'updateMany'
    | 'updateManyAndReturn'
    | 'upsert'
    | 'delete'
    | 'deleteMany'
    | 'executeRaw'
    | 'queryRaw'
    | 'aggregate'
    | 'count'
    | 'runCommandRaw'
    | 'findRaw'
    | 'groupBy'

  /**
   * These options are being passed into the middleware as "params"
   */
  export type MiddlewareParams = {
    model?: ModelName
    action: PrismaAction
    args: any
    dataPath: string[]
    runInTransaction: boolean
  }

  /**
   * The `T` type makes sure, that the `return proceed` is not forgotten in the middleware implementation
   */
  export type Middleware<T = any> = (
    params: MiddlewareParams,
    next: (params: MiddlewareParams) => $Utils.JsPromise<T>,
  ) => $Utils.JsPromise<T>

  // tested in getLogLevel.test.ts
  export function getLogLevel(log: Array<LogLevel | LogDefinition>): LogLevel | undefined;

  /**
   * `PrismaClient` proxy available in interactive transactions.
   */
  export type TransactionClient = Omit<Prisma.DefaultPrismaClient, runtime.ITXClientDenyList>

  export type Datasource = {
    url?: string
  }

  /**
   * Count Types
   */


  /**
   * Count Type UsersCountOutputType
   */

  export type UsersCountOutputType = {
    TrainingModels: number
    UserCredits: number
    Images: number
    Transactions: number
  }

  export type UsersCountOutputTypeSelect<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    TrainingModels?: boolean | UsersCountOutputTypeCountTrainingModelsArgs
    UserCredits?: boolean | UsersCountOutputTypeCountUserCreditsArgs
    Images?: boolean | UsersCountOutputTypeCountImagesArgs
    Transactions?: boolean | UsersCountOutputTypeCountTransactionsArgs
  }

  // Custom InputTypes
  /**
   * UsersCountOutputType without action
   */
  export type UsersCountOutputTypeDefaultArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UsersCountOutputType
     */
    select?: UsersCountOutputTypeSelect<ExtArgs> | null
  }

  /**
   * UsersCountOutputType without action
   */
  export type UsersCountOutputTypeCountTrainingModelsArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: TrainingModelsWhereInput
  }

  /**
   * UsersCountOutputType without action
   */
  export type UsersCountOutputTypeCountUserCreditsArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: UserCreditsWhereInput
  }

  /**
   * UsersCountOutputType without action
   */
  export type UsersCountOutputTypeCountImagesArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: ImagesWhereInput
  }

  /**
   * UsersCountOutputType without action
   */
  export type UsersCountOutputTypeCountTransactionsArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: TransactionsWhereInput
  }


  /**
   * Count Type TrainingModelsCountOutputType
   */

  export type TrainingModelsCountOutputType = {
    Images: number
  }

  export type TrainingModelsCountOutputTypeSelect<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    Images?: boolean | TrainingModelsCountOutputTypeCountImagesArgs
  }

  // Custom InputTypes
  /**
   * TrainingModelsCountOutputType without action
   */
  export type TrainingModelsCountOutputTypeDefaultArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModelsCountOutputType
     */
    select?: TrainingModelsCountOutputTypeSelect<ExtArgs> | null
  }

  /**
   * TrainingModelsCountOutputType without action
   */
  export type TrainingModelsCountOutputTypeCountImagesArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: ImagesWhereInput
  }


  /**
   * Count Type PacksCountOutputType
   */

  export type PacksCountOutputType = {
    Images: number
  }

  export type PacksCountOutputTypeSelect<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    Images?: boolean | PacksCountOutputTypeCountImagesArgs
  }

  // Custom InputTypes
  /**
   * PacksCountOutputType without action
   */
  export type PacksCountOutputTypeDefaultArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the PacksCountOutputType
     */
    select?: PacksCountOutputTypeSelect<ExtArgs> | null
  }

  /**
   * PacksCountOutputType without action
   */
  export type PacksCountOutputTypeCountImagesArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: ImagesWhereInput
  }


  /**
   * Models
   */

  /**
   * Model Users
   */

  export type AggregateUsers = {
    _count: UsersCountAggregateOutputType | null
    _avg: UsersAvgAggregateOutputType | null
    _sum: UsersSumAggregateOutputType | null
    _min: UsersMinAggregateOutputType | null
    _max: UsersMaxAggregateOutputType | null
  }

  export type UsersAvgAggregateOutputType = {
    id: number | null
  }

  export type UsersSumAggregateOutputType = {
    id: number | null
  }

  export type UsersMinAggregateOutputType = {
    id: number | null
    pid: string | null
    email: string | null
    password: string | null
    api_key: string | null
    name: string | null
    reset_token: string | null
    reset_sent_at: Date | null
    email_verification_token: string | null
    email_verification_sent_at: Date | null
    email_verified_at: Date | null
    magicLink_token: string | null
    magicLink_expiration: Date | null
  }

  export type UsersMaxAggregateOutputType = {
    id: number | null
    pid: string | null
    email: string | null
    password: string | null
    api_key: string | null
    name: string | null
    reset_token: string | null
    reset_sent_at: Date | null
    email_verification_token: string | null
    email_verification_sent_at: Date | null
    email_verified_at: Date | null
    magicLink_token: string | null
    magicLink_expiration: Date | null
  }

  export type UsersCountAggregateOutputType = {
    id: number
    pid: number
    email: number
    password: number
    api_key: number
    name: number
    reset_token: number
    reset_sent_at: number
    email_verification_token: number
    email_verification_sent_at: number
    email_verified_at: number
    magicLink_token: number
    magicLink_expiration: number
    _all: number
  }


  export type UsersAvgAggregateInputType = {
    id?: true
  }

  export type UsersSumAggregateInputType = {
    id?: true
  }

  export type UsersMinAggregateInputType = {
    id?: true
    pid?: true
    email?: true
    password?: true
    api_key?: true
    name?: true
    reset_token?: true
    reset_sent_at?: true
    email_verification_token?: true
    email_verification_sent_at?: true
    email_verified_at?: true
    magicLink_token?: true
    magicLink_expiration?: true
  }

  export type UsersMaxAggregateInputType = {
    id?: true
    pid?: true
    email?: true
    password?: true
    api_key?: true
    name?: true
    reset_token?: true
    reset_sent_at?: true
    email_verification_token?: true
    email_verification_sent_at?: true
    email_verified_at?: true
    magicLink_token?: true
    magicLink_expiration?: true
  }

  export type UsersCountAggregateInputType = {
    id?: true
    pid?: true
    email?: true
    password?: true
    api_key?: true
    name?: true
    reset_token?: true
    reset_sent_at?: true
    email_verification_token?: true
    email_verification_sent_at?: true
    email_verified_at?: true
    magicLink_token?: true
    magicLink_expiration?: true
    _all?: true
  }

  export type UsersAggregateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which Users to aggregate.
     */
    where?: UsersWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Users to fetch.
     */
    orderBy?: UsersOrderByWithRelationInput | UsersOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the start position
     */
    cursor?: UsersWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Users from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Users.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Count returned Users
    **/
    _count?: true | UsersCountAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to average
    **/
    _avg?: UsersAvgAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to sum
    **/
    _sum?: UsersSumAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the minimum value
    **/
    _min?: UsersMinAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the maximum value
    **/
    _max?: UsersMaxAggregateInputType
  }

  export type GetUsersAggregateType<T extends UsersAggregateArgs> = {
        [P in keyof T & keyof AggregateUsers]: P extends '_count' | 'count'
      ? T[P] extends true
        ? number
        : GetScalarType<T[P], AggregateUsers[P]>
      : GetScalarType<T[P], AggregateUsers[P]>
  }




  export type UsersGroupByArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: UsersWhereInput
    orderBy?: UsersOrderByWithAggregationInput | UsersOrderByWithAggregationInput[]
    by: UsersScalarFieldEnum[] | UsersScalarFieldEnum
    having?: UsersScalarWhereWithAggregatesInput
    take?: number
    skip?: number
    _count?: UsersCountAggregateInputType | true
    _avg?: UsersAvgAggregateInputType
    _sum?: UsersSumAggregateInputType
    _min?: UsersMinAggregateInputType
    _max?: UsersMaxAggregateInputType
  }

  export type UsersGroupByOutputType = {
    id: number
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token: string | null
    reset_sent_at: Date | null
    email_verification_token: string | null
    email_verification_sent_at: Date | null
    email_verified_at: Date | null
    magicLink_token: string | null
    magicLink_expiration: Date | null
    _count: UsersCountAggregateOutputType | null
    _avg: UsersAvgAggregateOutputType | null
    _sum: UsersSumAggregateOutputType | null
    _min: UsersMinAggregateOutputType | null
    _max: UsersMaxAggregateOutputType | null
  }

  type GetUsersGroupByPayload<T extends UsersGroupByArgs> = Prisma.PrismaPromise<
    Array<
      PickEnumerable<UsersGroupByOutputType, T['by']> &
        {
          [P in ((keyof T) & (keyof UsersGroupByOutputType))]: P extends '_count'
            ? T[P] extends boolean
              ? number
              : GetScalarType<T[P], UsersGroupByOutputType[P]>
            : GetScalarType<T[P], UsersGroupByOutputType[P]>
        }
      >
    >


  export type UsersSelect<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    email?: boolean
    password?: boolean
    api_key?: boolean
    name?: boolean
    reset_token?: boolean
    reset_sent_at?: boolean
    email_verification_token?: boolean
    email_verification_sent_at?: boolean
    email_verified_at?: boolean
    magicLink_token?: boolean
    magicLink_expiration?: boolean
    TrainingModels?: boolean | Users$TrainingModelsArgs<ExtArgs>
    UserCredits?: boolean | Users$UserCreditsArgs<ExtArgs>
    Images?: boolean | Users$ImagesArgs<ExtArgs>
    Transactions?: boolean | Users$TransactionsArgs<ExtArgs>
    _count?: boolean | UsersCountOutputTypeDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["users"]>

  export type UsersSelectCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    email?: boolean
    password?: boolean
    api_key?: boolean
    name?: boolean
    reset_token?: boolean
    reset_sent_at?: boolean
    email_verification_token?: boolean
    email_verification_sent_at?: boolean
    email_verified_at?: boolean
    magicLink_token?: boolean
    magicLink_expiration?: boolean
  }, ExtArgs["result"]["users"]>

  export type UsersSelectUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    email?: boolean
    password?: boolean
    api_key?: boolean
    name?: boolean
    reset_token?: boolean
    reset_sent_at?: boolean
    email_verification_token?: boolean
    email_verification_sent_at?: boolean
    email_verified_at?: boolean
    magicLink_token?: boolean
    magicLink_expiration?: boolean
  }, ExtArgs["result"]["users"]>

  export type UsersSelectScalar = {
    id?: boolean
    pid?: boolean
    email?: boolean
    password?: boolean
    api_key?: boolean
    name?: boolean
    reset_token?: boolean
    reset_sent_at?: boolean
    email_verification_token?: boolean
    email_verification_sent_at?: boolean
    email_verified_at?: boolean
    magicLink_token?: boolean
    magicLink_expiration?: boolean
  }

  export type UsersOmit<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetOmit<"id" | "pid" | "email" | "password" | "api_key" | "name" | "reset_token" | "reset_sent_at" | "email_verification_token" | "email_verification_sent_at" | "email_verified_at" | "magicLink_token" | "magicLink_expiration", ExtArgs["result"]["users"]>
  export type UsersInclude<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    TrainingModels?: boolean | Users$TrainingModelsArgs<ExtArgs>
    UserCredits?: boolean | Users$UserCreditsArgs<ExtArgs>
    Images?: boolean | Users$ImagesArgs<ExtArgs>
    Transactions?: boolean | Users$TransactionsArgs<ExtArgs>
    _count?: boolean | UsersCountOutputTypeDefaultArgs<ExtArgs>
  }
  export type UsersIncludeCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {}
  export type UsersIncludeUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {}

  export type $UsersPayload<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    name: "Users"
    objects: {
      TrainingModels: Prisma.$TrainingModelsPayload<ExtArgs>[]
      UserCredits: Prisma.$UserCreditsPayload<ExtArgs>[]
      Images: Prisma.$ImagesPayload<ExtArgs>[]
      Transactions: Prisma.$TransactionsPayload<ExtArgs>[]
    }
    scalars: $Extensions.GetPayloadResult<{
      id: number
      pid: string
      email: string
      password: string
      api_key: string
      name: string
      reset_token: string | null
      reset_sent_at: Date | null
      email_verification_token: string | null
      email_verification_sent_at: Date | null
      email_verified_at: Date | null
      magicLink_token: string | null
      magicLink_expiration: Date | null
    }, ExtArgs["result"]["users"]>
    composites: {}
  }

  type UsersGetPayload<S extends boolean | null | undefined | UsersDefaultArgs> = $Result.GetResult<Prisma.$UsersPayload, S>

  type UsersCountArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> =
    Omit<UsersFindManyArgs, 'select' | 'include' | 'distinct' | 'omit'> & {
      select?: UsersCountAggregateInputType | true
    }

  export interface UsersDelegate<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> {
    [K: symbol]: { types: Prisma.TypeMap<ExtArgs>['model']['Users'], meta: { name: 'Users' } }
    /**
     * Find zero or one Users that matches the filter.
     * @param {UsersFindUniqueArgs} args - Arguments to find a Users
     * @example
     * // Get one Users
     * const users = await prisma.users.findUnique({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUnique<T extends UsersFindUniqueArgs>(args: SelectSubset<T, UsersFindUniqueArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "findUnique", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find one Users that matches the filter or throw an error with `error.code='P2025'`
     * if no matches were found.
     * @param {UsersFindUniqueOrThrowArgs} args - Arguments to find a Users
     * @example
     * // Get one Users
     * const users = await prisma.users.findUniqueOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUniqueOrThrow<T extends UsersFindUniqueOrThrowArgs>(args: SelectSubset<T, UsersFindUniqueOrThrowArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first Users that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UsersFindFirstArgs} args - Arguments to find a Users
     * @example
     * // Get one Users
     * const users = await prisma.users.findFirst({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirst<T extends UsersFindFirstArgs>(args?: SelectSubset<T, UsersFindFirstArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "findFirst", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first Users that matches the filter or
     * throw `PrismaKnownClientError` with `P2025` code if no matches were found.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UsersFindFirstOrThrowArgs} args - Arguments to find a Users
     * @example
     * // Get one Users
     * const users = await prisma.users.findFirstOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirstOrThrow<T extends UsersFindFirstOrThrowArgs>(args?: SelectSubset<T, UsersFindFirstOrThrowArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "findFirstOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find zero or more Users that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UsersFindManyArgs} args - Arguments to filter and select certain fields only.
     * @example
     * // Get all Users
     * const users = await prisma.users.findMany()
     * 
     * // Get first 10 Users
     * const users = await prisma.users.findMany({ take: 10 })
     * 
     * // Only select the `id`
     * const usersWithIdOnly = await prisma.users.findMany({ select: { id: true } })
     * 
     */
    findMany<T extends UsersFindManyArgs>(args?: SelectSubset<T, UsersFindManyArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "findMany", GlobalOmitOptions>>

    /**
     * Create a Users.
     * @param {UsersCreateArgs} args - Arguments to create a Users.
     * @example
     * // Create one Users
     * const Users = await prisma.users.create({
     *   data: {
     *     // ... data to create a Users
     *   }
     * })
     * 
     */
    create<T extends UsersCreateArgs>(args: SelectSubset<T, UsersCreateArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "create", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Create many Users.
     * @param {UsersCreateManyArgs} args - Arguments to create many Users.
     * @example
     * // Create many Users
     * const users = await prisma.users.createMany({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     *     
     */
    createMany<T extends UsersCreateManyArgs>(args?: SelectSubset<T, UsersCreateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Create many Users and returns the data saved in the database.
     * @param {UsersCreateManyAndReturnArgs} args - Arguments to create many Users.
     * @example
     * // Create many Users
     * const users = await prisma.users.createManyAndReturn({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Create many Users and only return the `id`
     * const usersWithIdOnly = await prisma.users.createManyAndReturn({
     *   select: { id: true },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    createManyAndReturn<T extends UsersCreateManyAndReturnArgs>(args?: SelectSubset<T, UsersCreateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "createManyAndReturn", GlobalOmitOptions>>

    /**
     * Delete a Users.
     * @param {UsersDeleteArgs} args - Arguments to delete one Users.
     * @example
     * // Delete one Users
     * const Users = await prisma.users.delete({
     *   where: {
     *     // ... filter to delete one Users
     *   }
     * })
     * 
     */
    delete<T extends UsersDeleteArgs>(args: SelectSubset<T, UsersDeleteArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "delete", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Update one Users.
     * @param {UsersUpdateArgs} args - Arguments to update one Users.
     * @example
     * // Update one Users
     * const users = await prisma.users.update({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    update<T extends UsersUpdateArgs>(args: SelectSubset<T, UsersUpdateArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "update", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Delete zero or more Users.
     * @param {UsersDeleteManyArgs} args - Arguments to filter Users to delete.
     * @example
     * // Delete a few Users
     * const { count } = await prisma.users.deleteMany({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     * 
     */
    deleteMany<T extends UsersDeleteManyArgs>(args?: SelectSubset<T, UsersDeleteManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more Users.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UsersUpdateManyArgs} args - Arguments to update one or more rows.
     * @example
     * // Update many Users
     * const users = await prisma.users.updateMany({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    updateMany<T extends UsersUpdateManyArgs>(args: SelectSubset<T, UsersUpdateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more Users and returns the data updated in the database.
     * @param {UsersUpdateManyAndReturnArgs} args - Arguments to update many Users.
     * @example
     * // Update many Users
     * const users = await prisma.users.updateManyAndReturn({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Update zero or more Users and only return the `id`
     * const usersWithIdOnly = await prisma.users.updateManyAndReturn({
     *   select: { id: true },
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    updateManyAndReturn<T extends UsersUpdateManyAndReturnArgs>(args: SelectSubset<T, UsersUpdateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "updateManyAndReturn", GlobalOmitOptions>>

    /**
     * Create or update one Users.
     * @param {UsersUpsertArgs} args - Arguments to update or create a Users.
     * @example
     * // Update or create a Users
     * const users = await prisma.users.upsert({
     *   create: {
     *     // ... data to create a Users
     *   },
     *   update: {
     *     // ... in case it already exists, update
     *   },
     *   where: {
     *     // ... the filter for the Users we want to update
     *   }
     * })
     */
    upsert<T extends UsersUpsertArgs>(args: SelectSubset<T, UsersUpsertArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "upsert", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>


    /**
     * Count the number of Users.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UsersCountArgs} args - Arguments to filter Users to count.
     * @example
     * // Count the number of Users
     * const count = await prisma.users.count({
     *   where: {
     *     // ... the filter for the Users we want to count
     *   }
     * })
    **/
    count<T extends UsersCountArgs>(
      args?: Subset<T, UsersCountArgs>,
    ): Prisma.PrismaPromise<
      T extends $Utils.Record<'select', any>
        ? T['select'] extends true
          ? number
          : GetScalarType<T['select'], UsersCountAggregateOutputType>
        : number
    >

    /**
     * Allows you to perform aggregations operations on a Users.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UsersAggregateArgs} args - Select which aggregations you would like to apply and on what fields.
     * @example
     * // Ordered by age ascending
     * // Where email contains prisma.io
     * // Limited to the 10 users
     * const aggregations = await prisma.user.aggregate({
     *   _avg: {
     *     age: true,
     *   },
     *   where: {
     *     email: {
     *       contains: "prisma.io",
     *     },
     *   },
     *   orderBy: {
     *     age: "asc",
     *   },
     *   take: 10,
     * })
    **/
    aggregate<T extends UsersAggregateArgs>(args: Subset<T, UsersAggregateArgs>): Prisma.PrismaPromise<GetUsersAggregateType<T>>

    /**
     * Group by Users.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UsersGroupByArgs} args - Group by arguments.
     * @example
     * // Group by city, order by createdAt, get count
     * const result = await prisma.user.groupBy({
     *   by: ['city', 'createdAt'],
     *   orderBy: {
     *     createdAt: true
     *   },
     *   _count: {
     *     _all: true
     *   },
     * })
     * 
    **/
    groupBy<
      T extends UsersGroupByArgs,
      HasSelectOrTake extends Or<
        Extends<'skip', Keys<T>>,
        Extends<'take', Keys<T>>
      >,
      OrderByArg extends True extends HasSelectOrTake
        ? { orderBy: UsersGroupByArgs['orderBy'] }
        : { orderBy?: UsersGroupByArgs['orderBy'] },
      OrderFields extends ExcludeUnderscoreKeys<Keys<MaybeTupleToUnion<T['orderBy']>>>,
      ByFields extends MaybeTupleToUnion<T['by']>,
      ByValid extends Has<ByFields, OrderFields>,
      HavingFields extends GetHavingFields<T['having']>,
      HavingValid extends Has<ByFields, HavingFields>,
      ByEmpty extends T['by'] extends never[] ? True : False,
      InputErrors extends ByEmpty extends True
      ? `Error: "by" must not be empty.`
      : HavingValid extends False
      ? {
          [P in HavingFields]: P extends ByFields
            ? never
            : P extends string
            ? `Error: Field "${P}" used in "having" needs to be provided in "by".`
            : [
                Error,
                'Field ',
                P,
                ` in "having" needs to be provided in "by"`,
              ]
        }[HavingFields]
      : 'take' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "take", you also need to provide "orderBy"'
      : 'skip' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "skip", you also need to provide "orderBy"'
      : ByValid extends True
      ? {}
      : {
          [P in OrderFields]: P extends ByFields
            ? never
            : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
        }[OrderFields]
    >(args: SubsetIntersection<T, UsersGroupByArgs, OrderByArg> & InputErrors): {} extends InputErrors ? GetUsersGroupByPayload<T> : Prisma.PrismaPromise<InputErrors>
  /**
   * Fields of the Users model
   */
  readonly fields: UsersFieldRefs;
  }

  /**
   * The delegate class that acts as a "Promise-like" for Users.
   * Why is this prefixed with `Prisma__`?
   * Because we want to prevent naming conflicts as mentioned in
   * https://github.com/prisma/prisma-client-js/issues/707
   */
  export interface Prisma__UsersClient<T, Null = never, ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> extends Prisma.PrismaPromise<T> {
    readonly [Symbol.toStringTag]: "PrismaPromise"
    TrainingModels<T extends Users$TrainingModelsArgs<ExtArgs> = {}>(args?: Subset<T, Users$TrainingModelsArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "findMany", GlobalOmitOptions> | Null>
    UserCredits<T extends Users$UserCreditsArgs<ExtArgs> = {}>(args?: Subset<T, Users$UserCreditsArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "findMany", GlobalOmitOptions> | Null>
    Images<T extends Users$ImagesArgs<ExtArgs> = {}>(args?: Subset<T, Users$ImagesArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "findMany", GlobalOmitOptions> | Null>
    Transactions<T extends Users$TransactionsArgs<ExtArgs> = {}>(args?: Subset<T, Users$TransactionsArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "findMany", GlobalOmitOptions> | Null>
    /**
     * Attaches callbacks for the resolution and/or rejection of the Promise.
     * @param onfulfilled The callback to execute when the Promise is resolved.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of which ever callback is executed.
     */
    then<TResult1 = T, TResult2 = never>(onfulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): $Utils.JsPromise<TResult1 | TResult2>
    /**
     * Attaches a callback for only the rejection of the Promise.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of the callback.
     */
    catch<TResult = never>(onrejected?: ((reason: any) => TResult | PromiseLike<TResult>) | undefined | null): $Utils.JsPromise<T | TResult>
    /**
     * Attaches a callback that is invoked when the Promise is settled (fulfilled or rejected). The
     * resolved value cannot be modified from the callback.
     * @param onfinally The callback to execute when the Promise is settled (fulfilled or rejected).
     * @returns A Promise for the completion of the callback.
     */
    finally(onfinally?: (() => void) | undefined | null): $Utils.JsPromise<T>
  }




  /**
   * Fields of the Users model
   */ 
  interface UsersFieldRefs {
    readonly id: FieldRef<"Users", 'Int'>
    readonly pid: FieldRef<"Users", 'String'>
    readonly email: FieldRef<"Users", 'String'>
    readonly password: FieldRef<"Users", 'String'>
    readonly api_key: FieldRef<"Users", 'String'>
    readonly name: FieldRef<"Users", 'String'>
    readonly reset_token: FieldRef<"Users", 'String'>
    readonly reset_sent_at: FieldRef<"Users", 'DateTime'>
    readonly email_verification_token: FieldRef<"Users", 'String'>
    readonly email_verification_sent_at: FieldRef<"Users", 'DateTime'>
    readonly email_verified_at: FieldRef<"Users", 'DateTime'>
    readonly magicLink_token: FieldRef<"Users", 'String'>
    readonly magicLink_expiration: FieldRef<"Users", 'DateTime'>
  }
    

  // Custom InputTypes
  /**
   * Users findUnique
   */
  export type UsersFindUniqueArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UsersInclude<ExtArgs> | null
    /**
     * Filter, which Users to fetch.
     */
    where: UsersWhereUniqueInput
  }

  /**
   * Users findUniqueOrThrow
   */
  export type UsersFindUniqueOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UsersInclude<ExtArgs> | null
    /**
     * Filter, which Users to fetch.
     */
    where: UsersWhereUniqueInput
  }

  /**
   * Users findFirst
   */
  export type UsersFindFirstArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UsersInclude<ExtArgs> | null
    /**
     * Filter, which Users to fetch.
     */
    where?: UsersWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Users to fetch.
     */
    orderBy?: UsersOrderByWithRelationInput | UsersOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for Users.
     */
    cursor?: UsersWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Users from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Users.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of Users.
     */
    distinct?: UsersScalarFieldEnum | UsersScalarFieldEnum[]
  }

  /**
   * Users findFirstOrThrow
   */
  export type UsersFindFirstOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UsersInclude<ExtArgs> | null
    /**
     * Filter, which Users to fetch.
     */
    where?: UsersWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Users to fetch.
     */
    orderBy?: UsersOrderByWithRelationInput | UsersOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for Users.
     */
    cursor?: UsersWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Users from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Users.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of Users.
     */
    distinct?: UsersScalarFieldEnum | UsersScalarFieldEnum[]
  }

  /**
   * Users findMany
   */
  export type UsersFindManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UsersInclude<ExtArgs> | null
    /**
     * Filter, which Users to fetch.
     */
    where?: UsersWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Users to fetch.
     */
    orderBy?: UsersOrderByWithRelationInput | UsersOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for listing Users.
     */
    cursor?: UsersWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Users from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Users.
     */
    skip?: number
    distinct?: UsersScalarFieldEnum | UsersScalarFieldEnum[]
  }

  /**
   * Users create
   */
  export type UsersCreateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UsersInclude<ExtArgs> | null
    /**
     * The data needed to create a Users.
     */
    data: XOR<UsersCreateInput, UsersUncheckedCreateInput>
  }

  /**
   * Users createMany
   */
  export type UsersCreateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to create many Users.
     */
    data: UsersCreateManyInput | UsersCreateManyInput[]
    skipDuplicates?: boolean
  }

  /**
   * Users createManyAndReturn
   */
  export type UsersCreateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelectCreateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * The data used to create many Users.
     */
    data: UsersCreateManyInput | UsersCreateManyInput[]
    skipDuplicates?: boolean
  }

  /**
   * Users update
   */
  export type UsersUpdateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UsersInclude<ExtArgs> | null
    /**
     * The data needed to update a Users.
     */
    data: XOR<UsersUpdateInput, UsersUncheckedUpdateInput>
    /**
     * Choose, which Users to update.
     */
    where: UsersWhereUniqueInput
  }

  /**
   * Users updateMany
   */
  export type UsersUpdateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to update Users.
     */
    data: XOR<UsersUpdateManyMutationInput, UsersUncheckedUpdateManyInput>
    /**
     * Filter which Users to update
     */
    where?: UsersWhereInput
    /**
     * Limit how many Users to update.
     */
    limit?: number
  }

  /**
   * Users updateManyAndReturn
   */
  export type UsersUpdateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelectUpdateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * The data used to update Users.
     */
    data: XOR<UsersUpdateManyMutationInput, UsersUncheckedUpdateManyInput>
    /**
     * Filter which Users to update
     */
    where?: UsersWhereInput
    /**
     * Limit how many Users to update.
     */
    limit?: number
  }

  /**
   * Users upsert
   */
  export type UsersUpsertArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UsersInclude<ExtArgs> | null
    /**
     * The filter to search for the Users to update in case it exists.
     */
    where: UsersWhereUniqueInput
    /**
     * In case the Users found by the `where` argument doesn't exist, create a new Users with this data.
     */
    create: XOR<UsersCreateInput, UsersUncheckedCreateInput>
    /**
     * In case the Users was found with the provided `where` argument, update it with this data.
     */
    update: XOR<UsersUpdateInput, UsersUncheckedUpdateInput>
  }

  /**
   * Users delete
   */
  export type UsersDeleteArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UsersInclude<ExtArgs> | null
    /**
     * Filter which Users to delete.
     */
    where: UsersWhereUniqueInput
  }

  /**
   * Users deleteMany
   */
  export type UsersDeleteManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which Users to delete
     */
    where?: UsersWhereInput
    /**
     * Limit how many Users to delete.
     */
    limit?: number
  }

  /**
   * Users.TrainingModels
   */
  export type Users$TrainingModelsArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
    where?: TrainingModelsWhereInput
    orderBy?: TrainingModelsOrderByWithRelationInput | TrainingModelsOrderByWithRelationInput[]
    cursor?: TrainingModelsWhereUniqueInput
    take?: number
    skip?: number
    distinct?: TrainingModelsScalarFieldEnum | TrainingModelsScalarFieldEnum[]
  }

  /**
   * Users.UserCredits
   */
  export type Users$UserCreditsArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
    where?: UserCreditsWhereInput
    orderBy?: UserCreditsOrderByWithRelationInput | UserCreditsOrderByWithRelationInput[]
    cursor?: UserCreditsWhereUniqueInput
    take?: number
    skip?: number
    distinct?: UserCreditsScalarFieldEnum | UserCreditsScalarFieldEnum[]
  }

  /**
   * Users.Images
   */
  export type Users$ImagesArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    where?: ImagesWhereInput
    orderBy?: ImagesOrderByWithRelationInput | ImagesOrderByWithRelationInput[]
    cursor?: ImagesWhereUniqueInput
    take?: number
    skip?: number
    distinct?: ImagesScalarFieldEnum | ImagesScalarFieldEnum[]
  }

  /**
   * Users.Transactions
   */
  export type Users$TransactionsArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
    where?: TransactionsWhereInput
    orderBy?: TransactionsOrderByWithRelationInput | TransactionsOrderByWithRelationInput[]
    cursor?: TransactionsWhereUniqueInput
    take?: number
    skip?: number
    distinct?: TransactionsScalarFieldEnum | TransactionsScalarFieldEnum[]
  }

  /**
   * Users without action
   */
  export type UsersDefaultArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Users
     */
    select?: UsersSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Users
     */
    omit?: UsersOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UsersInclude<ExtArgs> | null
  }


  /**
   * Model TrainingModels
   */

  export type AggregateTrainingModels = {
    _count: TrainingModelsCountAggregateOutputType | null
    _avg: TrainingModelsAvgAggregateOutputType | null
    _sum: TrainingModelsSumAggregateOutputType | null
    _min: TrainingModelsMinAggregateOutputType | null
    _max: TrainingModelsMaxAggregateOutputType | null
  }

  export type TrainingModelsAvgAggregateOutputType = {
    id: number | null
    user_id: number | null
    age: number | null
    steps: number | null
  }

  export type TrainingModelsSumAggregateOutputType = {
    id: number | null
    user_id: number | null
    age: number | null
    steps: number | null
  }

  export type TrainingModelsMinAggregateOutputType = {
    id: number | null
    pid: string | null
    user_id: number | null
    name: string | null
    age: number | null
    sex: $Enums.Sex | null
    ethnicity: $Enums.Ethnicity | null
    basedOn: $Enums.BasedOn | null
    eye_color: $Enums.EyeColor | null
    bald: boolean | null
    steps: number | null
    create_mask: boolean | null
    is_style: boolean | null
    trigger_word: string | null
    tensor_path: string | null
    thumbnail: string | null
    training_status: $Enums.Status | null
    fal_ai_request_id: string | null
    s3_key: string | null
    is_verified: boolean | null
    deleted_at: Date | null
    created_at: Date | null
    updated_at: Date | null
  }

  export type TrainingModelsMaxAggregateOutputType = {
    id: number | null
    pid: string | null
    user_id: number | null
    name: string | null
    age: number | null
    sex: $Enums.Sex | null
    ethnicity: $Enums.Ethnicity | null
    basedOn: $Enums.BasedOn | null
    eye_color: $Enums.EyeColor | null
    bald: boolean | null
    steps: number | null
    create_mask: boolean | null
    is_style: boolean | null
    trigger_word: string | null
    tensor_path: string | null
    thumbnail: string | null
    training_status: $Enums.Status | null
    fal_ai_request_id: string | null
    s3_key: string | null
    is_verified: boolean | null
    deleted_at: Date | null
    created_at: Date | null
    updated_at: Date | null
  }

  export type TrainingModelsCountAggregateOutputType = {
    id: number
    pid: number
    user_id: number
    name: number
    age: number
    sex: number
    ethnicity: number
    basedOn: number
    eye_color: number
    bald: number
    steps: number
    create_mask: number
    is_style: number
    trigger_word: number
    tensor_path: number
    thumbnail: number
    training_status: number
    fal_output: number
    training_images: number
    fal_ai_request_id: number
    s3_key: number
    is_verified: number
    deleted_at: number
    created_at: number
    updated_at: number
    _all: number
  }


  export type TrainingModelsAvgAggregateInputType = {
    id?: true
    user_id?: true
    age?: true
    steps?: true
  }

  export type TrainingModelsSumAggregateInputType = {
    id?: true
    user_id?: true
    age?: true
    steps?: true
  }

  export type TrainingModelsMinAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    name?: true
    age?: true
    sex?: true
    ethnicity?: true
    basedOn?: true
    eye_color?: true
    bald?: true
    steps?: true
    create_mask?: true
    is_style?: true
    trigger_word?: true
    tensor_path?: true
    thumbnail?: true
    training_status?: true
    fal_ai_request_id?: true
    s3_key?: true
    is_verified?: true
    deleted_at?: true
    created_at?: true
    updated_at?: true
  }

  export type TrainingModelsMaxAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    name?: true
    age?: true
    sex?: true
    ethnicity?: true
    basedOn?: true
    eye_color?: true
    bald?: true
    steps?: true
    create_mask?: true
    is_style?: true
    trigger_word?: true
    tensor_path?: true
    thumbnail?: true
    training_status?: true
    fal_ai_request_id?: true
    s3_key?: true
    is_verified?: true
    deleted_at?: true
    created_at?: true
    updated_at?: true
  }

  export type TrainingModelsCountAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    name?: true
    age?: true
    sex?: true
    ethnicity?: true
    basedOn?: true
    eye_color?: true
    bald?: true
    steps?: true
    create_mask?: true
    is_style?: true
    trigger_word?: true
    tensor_path?: true
    thumbnail?: true
    training_status?: true
    fal_output?: true
    training_images?: true
    fal_ai_request_id?: true
    s3_key?: true
    is_verified?: true
    deleted_at?: true
    created_at?: true
    updated_at?: true
    _all?: true
  }

  export type TrainingModelsAggregateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which TrainingModels to aggregate.
     */
    where?: TrainingModelsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of TrainingModels to fetch.
     */
    orderBy?: TrainingModelsOrderByWithRelationInput | TrainingModelsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the start position
     */
    cursor?: TrainingModelsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` TrainingModels from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` TrainingModels.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Count returned TrainingModels
    **/
    _count?: true | TrainingModelsCountAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to average
    **/
    _avg?: TrainingModelsAvgAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to sum
    **/
    _sum?: TrainingModelsSumAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the minimum value
    **/
    _min?: TrainingModelsMinAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the maximum value
    **/
    _max?: TrainingModelsMaxAggregateInputType
  }

  export type GetTrainingModelsAggregateType<T extends TrainingModelsAggregateArgs> = {
        [P in keyof T & keyof AggregateTrainingModels]: P extends '_count' | 'count'
      ? T[P] extends true
        ? number
        : GetScalarType<T[P], AggregateTrainingModels[P]>
      : GetScalarType<T[P], AggregateTrainingModels[P]>
  }




  export type TrainingModelsGroupByArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: TrainingModelsWhereInput
    orderBy?: TrainingModelsOrderByWithAggregationInput | TrainingModelsOrderByWithAggregationInput[]
    by: TrainingModelsScalarFieldEnum[] | TrainingModelsScalarFieldEnum
    having?: TrainingModelsScalarWhereWithAggregatesInput
    take?: number
    skip?: number
    _count?: TrainingModelsCountAggregateInputType | true
    _avg?: TrainingModelsAvgAggregateInputType
    _sum?: TrainingModelsSumAggregateInputType
    _min?: TrainingModelsMinAggregateInputType
    _max?: TrainingModelsMaxAggregateInputType
  }

  export type TrainingModelsGroupByOutputType = {
    id: number
    pid: string
    user_id: number
    name: string
    age: number
    sex: $Enums.Sex
    ethnicity: $Enums.Ethnicity
    basedOn: $Enums.BasedOn
    eye_color: $Enums.EyeColor
    bald: boolean
    steps: number
    create_mask: boolean
    is_style: boolean
    trigger_word: string
    tensor_path: string | null
    thumbnail: string | null
    training_status: $Enums.Status
    fal_output: JsonValue | null
    training_images: JsonValue | null
    fal_ai_request_id: string | null
    s3_key: string
    is_verified: boolean
    deleted_at: Date | null
    created_at: Date
    updated_at: Date
    _count: TrainingModelsCountAggregateOutputType | null
    _avg: TrainingModelsAvgAggregateOutputType | null
    _sum: TrainingModelsSumAggregateOutputType | null
    _min: TrainingModelsMinAggregateOutputType | null
    _max: TrainingModelsMaxAggregateOutputType | null
  }

  type GetTrainingModelsGroupByPayload<T extends TrainingModelsGroupByArgs> = Prisma.PrismaPromise<
    Array<
      PickEnumerable<TrainingModelsGroupByOutputType, T['by']> &
        {
          [P in ((keyof T) & (keyof TrainingModelsGroupByOutputType))]: P extends '_count'
            ? T[P] extends boolean
              ? number
              : GetScalarType<T[P], TrainingModelsGroupByOutputType[P]>
            : GetScalarType<T[P], TrainingModelsGroupByOutputType[P]>
        }
      >
    >


  export type TrainingModelsSelect<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    name?: boolean
    age?: boolean
    sex?: boolean
    ethnicity?: boolean
    basedOn?: boolean
    eye_color?: boolean
    bald?: boolean
    steps?: boolean
    create_mask?: boolean
    is_style?: boolean
    trigger_word?: boolean
    tensor_path?: boolean
    thumbnail?: boolean
    training_status?: boolean
    fal_output?: boolean
    training_images?: boolean
    fal_ai_request_id?: boolean
    s3_key?: boolean
    is_verified?: boolean
    deleted_at?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
    Images?: boolean | TrainingModels$ImagesArgs<ExtArgs>
    _count?: boolean | TrainingModelsCountOutputTypeDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["trainingModels"]>

  export type TrainingModelsSelectCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    name?: boolean
    age?: boolean
    sex?: boolean
    ethnicity?: boolean
    basedOn?: boolean
    eye_color?: boolean
    bald?: boolean
    steps?: boolean
    create_mask?: boolean
    is_style?: boolean
    trigger_word?: boolean
    tensor_path?: boolean
    thumbnail?: boolean
    training_status?: boolean
    fal_output?: boolean
    training_images?: boolean
    fal_ai_request_id?: boolean
    s3_key?: boolean
    is_verified?: boolean
    deleted_at?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["trainingModels"]>

  export type TrainingModelsSelectUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    name?: boolean
    age?: boolean
    sex?: boolean
    ethnicity?: boolean
    basedOn?: boolean
    eye_color?: boolean
    bald?: boolean
    steps?: boolean
    create_mask?: boolean
    is_style?: boolean
    trigger_word?: boolean
    tensor_path?: boolean
    thumbnail?: boolean
    training_status?: boolean
    fal_output?: boolean
    training_images?: boolean
    fal_ai_request_id?: boolean
    s3_key?: boolean
    is_verified?: boolean
    deleted_at?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["trainingModels"]>

  export type TrainingModelsSelectScalar = {
    id?: boolean
    pid?: boolean
    user_id?: boolean
    name?: boolean
    age?: boolean
    sex?: boolean
    ethnicity?: boolean
    basedOn?: boolean
    eye_color?: boolean
    bald?: boolean
    steps?: boolean
    create_mask?: boolean
    is_style?: boolean
    trigger_word?: boolean
    tensor_path?: boolean
    thumbnail?: boolean
    training_status?: boolean
    fal_output?: boolean
    training_images?: boolean
    fal_ai_request_id?: boolean
    s3_key?: boolean
    is_verified?: boolean
    deleted_at?: boolean
    created_at?: boolean
    updated_at?: boolean
  }

  export type TrainingModelsOmit<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetOmit<"id" | "pid" | "user_id" | "name" | "age" | "sex" | "ethnicity" | "basedOn" | "eye_color" | "bald" | "steps" | "create_mask" | "is_style" | "trigger_word" | "tensor_path" | "thumbnail" | "training_status" | "fal_output" | "training_images" | "fal_ai_request_id" | "s3_key" | "is_verified" | "deleted_at" | "created_at" | "updated_at", ExtArgs["result"]["trainingModels"]>
  export type TrainingModelsInclude<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
    Images?: boolean | TrainingModels$ImagesArgs<ExtArgs>
    _count?: boolean | TrainingModelsCountOutputTypeDefaultArgs<ExtArgs>
  }
  export type TrainingModelsIncludeCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }
  export type TrainingModelsIncludeUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }

  export type $TrainingModelsPayload<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    name: "TrainingModels"
    objects: {
      user: Prisma.$UsersPayload<ExtArgs>
      Images: Prisma.$ImagesPayload<ExtArgs>[]
    }
    scalars: $Extensions.GetPayloadResult<{
      id: number
      pid: string
      user_id: number
      name: string
      age: number
      sex: $Enums.Sex
      ethnicity: $Enums.Ethnicity
      basedOn: $Enums.BasedOn
      eye_color: $Enums.EyeColor
      bald: boolean
      steps: number
      create_mask: boolean
      is_style: boolean
      trigger_word: string
      tensor_path: string | null
      thumbnail: string | null
      training_status: $Enums.Status
      fal_output: Prisma.JsonValue | null
      training_images: Prisma.JsonValue | null
      fal_ai_request_id: string | null
      s3_key: string
      is_verified: boolean
      deleted_at: Date | null
      created_at: Date
      updated_at: Date
    }, ExtArgs["result"]["trainingModels"]>
    composites: {}
  }

  type TrainingModelsGetPayload<S extends boolean | null | undefined | TrainingModelsDefaultArgs> = $Result.GetResult<Prisma.$TrainingModelsPayload, S>

  type TrainingModelsCountArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> =
    Omit<TrainingModelsFindManyArgs, 'select' | 'include' | 'distinct' | 'omit'> & {
      select?: TrainingModelsCountAggregateInputType | true
    }

  export interface TrainingModelsDelegate<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> {
    [K: symbol]: { types: Prisma.TypeMap<ExtArgs>['model']['TrainingModels'], meta: { name: 'TrainingModels' } }
    /**
     * Find zero or one TrainingModels that matches the filter.
     * @param {TrainingModelsFindUniqueArgs} args - Arguments to find a TrainingModels
     * @example
     * // Get one TrainingModels
     * const trainingModels = await prisma.trainingModels.findUnique({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUnique<T extends TrainingModelsFindUniqueArgs>(args: SelectSubset<T, TrainingModelsFindUniqueArgs<ExtArgs>>): Prisma__TrainingModelsClient<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "findUnique", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find one TrainingModels that matches the filter or throw an error with `error.code='P2025'`
     * if no matches were found.
     * @param {TrainingModelsFindUniqueOrThrowArgs} args - Arguments to find a TrainingModels
     * @example
     * // Get one TrainingModels
     * const trainingModels = await prisma.trainingModels.findUniqueOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUniqueOrThrow<T extends TrainingModelsFindUniqueOrThrowArgs>(args: SelectSubset<T, TrainingModelsFindUniqueOrThrowArgs<ExtArgs>>): Prisma__TrainingModelsClient<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first TrainingModels that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TrainingModelsFindFirstArgs} args - Arguments to find a TrainingModels
     * @example
     * // Get one TrainingModels
     * const trainingModels = await prisma.trainingModels.findFirst({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirst<T extends TrainingModelsFindFirstArgs>(args?: SelectSubset<T, TrainingModelsFindFirstArgs<ExtArgs>>): Prisma__TrainingModelsClient<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "findFirst", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first TrainingModels that matches the filter or
     * throw `PrismaKnownClientError` with `P2025` code if no matches were found.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TrainingModelsFindFirstOrThrowArgs} args - Arguments to find a TrainingModels
     * @example
     * // Get one TrainingModels
     * const trainingModels = await prisma.trainingModels.findFirstOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirstOrThrow<T extends TrainingModelsFindFirstOrThrowArgs>(args?: SelectSubset<T, TrainingModelsFindFirstOrThrowArgs<ExtArgs>>): Prisma__TrainingModelsClient<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "findFirstOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find zero or more TrainingModels that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TrainingModelsFindManyArgs} args - Arguments to filter and select certain fields only.
     * @example
     * // Get all TrainingModels
     * const trainingModels = await prisma.trainingModels.findMany()
     * 
     * // Get first 10 TrainingModels
     * const trainingModels = await prisma.trainingModels.findMany({ take: 10 })
     * 
     * // Only select the `id`
     * const trainingModelsWithIdOnly = await prisma.trainingModels.findMany({ select: { id: true } })
     * 
     */
    findMany<T extends TrainingModelsFindManyArgs>(args?: SelectSubset<T, TrainingModelsFindManyArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "findMany", GlobalOmitOptions>>

    /**
     * Create a TrainingModels.
     * @param {TrainingModelsCreateArgs} args - Arguments to create a TrainingModels.
     * @example
     * // Create one TrainingModels
     * const TrainingModels = await prisma.trainingModels.create({
     *   data: {
     *     // ... data to create a TrainingModels
     *   }
     * })
     * 
     */
    create<T extends TrainingModelsCreateArgs>(args: SelectSubset<T, TrainingModelsCreateArgs<ExtArgs>>): Prisma__TrainingModelsClient<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "create", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Create many TrainingModels.
     * @param {TrainingModelsCreateManyArgs} args - Arguments to create many TrainingModels.
     * @example
     * // Create many TrainingModels
     * const trainingModels = await prisma.trainingModels.createMany({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     *     
     */
    createMany<T extends TrainingModelsCreateManyArgs>(args?: SelectSubset<T, TrainingModelsCreateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Create many TrainingModels and returns the data saved in the database.
     * @param {TrainingModelsCreateManyAndReturnArgs} args - Arguments to create many TrainingModels.
     * @example
     * // Create many TrainingModels
     * const trainingModels = await prisma.trainingModels.createManyAndReturn({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Create many TrainingModels and only return the `id`
     * const trainingModelsWithIdOnly = await prisma.trainingModels.createManyAndReturn({
     *   select: { id: true },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    createManyAndReturn<T extends TrainingModelsCreateManyAndReturnArgs>(args?: SelectSubset<T, TrainingModelsCreateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "createManyAndReturn", GlobalOmitOptions>>

    /**
     * Delete a TrainingModels.
     * @param {TrainingModelsDeleteArgs} args - Arguments to delete one TrainingModels.
     * @example
     * // Delete one TrainingModels
     * const TrainingModels = await prisma.trainingModels.delete({
     *   where: {
     *     // ... filter to delete one TrainingModels
     *   }
     * })
     * 
     */
    delete<T extends TrainingModelsDeleteArgs>(args: SelectSubset<T, TrainingModelsDeleteArgs<ExtArgs>>): Prisma__TrainingModelsClient<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "delete", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Update one TrainingModels.
     * @param {TrainingModelsUpdateArgs} args - Arguments to update one TrainingModels.
     * @example
     * // Update one TrainingModels
     * const trainingModels = await prisma.trainingModels.update({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    update<T extends TrainingModelsUpdateArgs>(args: SelectSubset<T, TrainingModelsUpdateArgs<ExtArgs>>): Prisma__TrainingModelsClient<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "update", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Delete zero or more TrainingModels.
     * @param {TrainingModelsDeleteManyArgs} args - Arguments to filter TrainingModels to delete.
     * @example
     * // Delete a few TrainingModels
     * const { count } = await prisma.trainingModels.deleteMany({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     * 
     */
    deleteMany<T extends TrainingModelsDeleteManyArgs>(args?: SelectSubset<T, TrainingModelsDeleteManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more TrainingModels.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TrainingModelsUpdateManyArgs} args - Arguments to update one or more rows.
     * @example
     * // Update many TrainingModels
     * const trainingModels = await prisma.trainingModels.updateMany({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    updateMany<T extends TrainingModelsUpdateManyArgs>(args: SelectSubset<T, TrainingModelsUpdateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more TrainingModels and returns the data updated in the database.
     * @param {TrainingModelsUpdateManyAndReturnArgs} args - Arguments to update many TrainingModels.
     * @example
     * // Update many TrainingModels
     * const trainingModels = await prisma.trainingModels.updateManyAndReturn({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Update zero or more TrainingModels and only return the `id`
     * const trainingModelsWithIdOnly = await prisma.trainingModels.updateManyAndReturn({
     *   select: { id: true },
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    updateManyAndReturn<T extends TrainingModelsUpdateManyAndReturnArgs>(args: SelectSubset<T, TrainingModelsUpdateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "updateManyAndReturn", GlobalOmitOptions>>

    /**
     * Create or update one TrainingModels.
     * @param {TrainingModelsUpsertArgs} args - Arguments to update or create a TrainingModels.
     * @example
     * // Update or create a TrainingModels
     * const trainingModels = await prisma.trainingModels.upsert({
     *   create: {
     *     // ... data to create a TrainingModels
     *   },
     *   update: {
     *     // ... in case it already exists, update
     *   },
     *   where: {
     *     // ... the filter for the TrainingModels we want to update
     *   }
     * })
     */
    upsert<T extends TrainingModelsUpsertArgs>(args: SelectSubset<T, TrainingModelsUpsertArgs<ExtArgs>>): Prisma__TrainingModelsClient<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "upsert", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>


    /**
     * Count the number of TrainingModels.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TrainingModelsCountArgs} args - Arguments to filter TrainingModels to count.
     * @example
     * // Count the number of TrainingModels
     * const count = await prisma.trainingModels.count({
     *   where: {
     *     // ... the filter for the TrainingModels we want to count
     *   }
     * })
    **/
    count<T extends TrainingModelsCountArgs>(
      args?: Subset<T, TrainingModelsCountArgs>,
    ): Prisma.PrismaPromise<
      T extends $Utils.Record<'select', any>
        ? T['select'] extends true
          ? number
          : GetScalarType<T['select'], TrainingModelsCountAggregateOutputType>
        : number
    >

    /**
     * Allows you to perform aggregations operations on a TrainingModels.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TrainingModelsAggregateArgs} args - Select which aggregations you would like to apply and on what fields.
     * @example
     * // Ordered by age ascending
     * // Where email contains prisma.io
     * // Limited to the 10 users
     * const aggregations = await prisma.user.aggregate({
     *   _avg: {
     *     age: true,
     *   },
     *   where: {
     *     email: {
     *       contains: "prisma.io",
     *     },
     *   },
     *   orderBy: {
     *     age: "asc",
     *   },
     *   take: 10,
     * })
    **/
    aggregate<T extends TrainingModelsAggregateArgs>(args: Subset<T, TrainingModelsAggregateArgs>): Prisma.PrismaPromise<GetTrainingModelsAggregateType<T>>

    /**
     * Group by TrainingModels.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TrainingModelsGroupByArgs} args - Group by arguments.
     * @example
     * // Group by city, order by createdAt, get count
     * const result = await prisma.user.groupBy({
     *   by: ['city', 'createdAt'],
     *   orderBy: {
     *     createdAt: true
     *   },
     *   _count: {
     *     _all: true
     *   },
     * })
     * 
    **/
    groupBy<
      T extends TrainingModelsGroupByArgs,
      HasSelectOrTake extends Or<
        Extends<'skip', Keys<T>>,
        Extends<'take', Keys<T>>
      >,
      OrderByArg extends True extends HasSelectOrTake
        ? { orderBy: TrainingModelsGroupByArgs['orderBy'] }
        : { orderBy?: TrainingModelsGroupByArgs['orderBy'] },
      OrderFields extends ExcludeUnderscoreKeys<Keys<MaybeTupleToUnion<T['orderBy']>>>,
      ByFields extends MaybeTupleToUnion<T['by']>,
      ByValid extends Has<ByFields, OrderFields>,
      HavingFields extends GetHavingFields<T['having']>,
      HavingValid extends Has<ByFields, HavingFields>,
      ByEmpty extends T['by'] extends never[] ? True : False,
      InputErrors extends ByEmpty extends True
      ? `Error: "by" must not be empty.`
      : HavingValid extends False
      ? {
          [P in HavingFields]: P extends ByFields
            ? never
            : P extends string
            ? `Error: Field "${P}" used in "having" needs to be provided in "by".`
            : [
                Error,
                'Field ',
                P,
                ` in "having" needs to be provided in "by"`,
              ]
        }[HavingFields]
      : 'take' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "take", you also need to provide "orderBy"'
      : 'skip' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "skip", you also need to provide "orderBy"'
      : ByValid extends True
      ? {}
      : {
          [P in OrderFields]: P extends ByFields
            ? never
            : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
        }[OrderFields]
    >(args: SubsetIntersection<T, TrainingModelsGroupByArgs, OrderByArg> & InputErrors): {} extends InputErrors ? GetTrainingModelsGroupByPayload<T> : Prisma.PrismaPromise<InputErrors>
  /**
   * Fields of the TrainingModels model
   */
  readonly fields: TrainingModelsFieldRefs;
  }

  /**
   * The delegate class that acts as a "Promise-like" for TrainingModels.
   * Why is this prefixed with `Prisma__`?
   * Because we want to prevent naming conflicts as mentioned in
   * https://github.com/prisma/prisma-client-js/issues/707
   */
  export interface Prisma__TrainingModelsClient<T, Null = never, ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> extends Prisma.PrismaPromise<T> {
    readonly [Symbol.toStringTag]: "PrismaPromise"
    user<T extends UsersDefaultArgs<ExtArgs> = {}>(args?: Subset<T, UsersDefaultArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions> | Null, Null, ExtArgs, GlobalOmitOptions>
    Images<T extends TrainingModels$ImagesArgs<ExtArgs> = {}>(args?: Subset<T, TrainingModels$ImagesArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "findMany", GlobalOmitOptions> | Null>
    /**
     * Attaches callbacks for the resolution and/or rejection of the Promise.
     * @param onfulfilled The callback to execute when the Promise is resolved.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of which ever callback is executed.
     */
    then<TResult1 = T, TResult2 = never>(onfulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): $Utils.JsPromise<TResult1 | TResult2>
    /**
     * Attaches a callback for only the rejection of the Promise.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of the callback.
     */
    catch<TResult = never>(onrejected?: ((reason: any) => TResult | PromiseLike<TResult>) | undefined | null): $Utils.JsPromise<T | TResult>
    /**
     * Attaches a callback that is invoked when the Promise is settled (fulfilled or rejected). The
     * resolved value cannot be modified from the callback.
     * @param onfinally The callback to execute when the Promise is settled (fulfilled or rejected).
     * @returns A Promise for the completion of the callback.
     */
    finally(onfinally?: (() => void) | undefined | null): $Utils.JsPromise<T>
  }




  /**
   * Fields of the TrainingModels model
   */ 
  interface TrainingModelsFieldRefs {
    readonly id: FieldRef<"TrainingModels", 'Int'>
    readonly pid: FieldRef<"TrainingModels", 'String'>
    readonly user_id: FieldRef<"TrainingModels", 'Int'>
    readonly name: FieldRef<"TrainingModels", 'String'>
    readonly age: FieldRef<"TrainingModels", 'Int'>
    readonly sex: FieldRef<"TrainingModels", 'Sex'>
    readonly ethnicity: FieldRef<"TrainingModels", 'Ethnicity'>
    readonly basedOn: FieldRef<"TrainingModels", 'BasedOn'>
    readonly eye_color: FieldRef<"TrainingModels", 'EyeColor'>
    readonly bald: FieldRef<"TrainingModels", 'Boolean'>
    readonly steps: FieldRef<"TrainingModels", 'Int'>
    readonly create_mask: FieldRef<"TrainingModels", 'Boolean'>
    readonly is_style: FieldRef<"TrainingModels", 'Boolean'>
    readonly trigger_word: FieldRef<"TrainingModels", 'String'>
    readonly tensor_path: FieldRef<"TrainingModels", 'String'>
    readonly thumbnail: FieldRef<"TrainingModels", 'String'>
    readonly training_status: FieldRef<"TrainingModels", 'Status'>
    readonly fal_output: FieldRef<"TrainingModels", 'Json'>
    readonly training_images: FieldRef<"TrainingModels", 'Json'>
    readonly fal_ai_request_id: FieldRef<"TrainingModels", 'String'>
    readonly s3_key: FieldRef<"TrainingModels", 'String'>
    readonly is_verified: FieldRef<"TrainingModels", 'Boolean'>
    readonly deleted_at: FieldRef<"TrainingModels", 'DateTime'>
    readonly created_at: FieldRef<"TrainingModels", 'DateTime'>
    readonly updated_at: FieldRef<"TrainingModels", 'DateTime'>
  }
    

  // Custom InputTypes
  /**
   * TrainingModels findUnique
   */
  export type TrainingModelsFindUniqueArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
    /**
     * Filter, which TrainingModels to fetch.
     */
    where: TrainingModelsWhereUniqueInput
  }

  /**
   * TrainingModels findUniqueOrThrow
   */
  export type TrainingModelsFindUniqueOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
    /**
     * Filter, which TrainingModels to fetch.
     */
    where: TrainingModelsWhereUniqueInput
  }

  /**
   * TrainingModels findFirst
   */
  export type TrainingModelsFindFirstArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
    /**
     * Filter, which TrainingModels to fetch.
     */
    where?: TrainingModelsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of TrainingModels to fetch.
     */
    orderBy?: TrainingModelsOrderByWithRelationInput | TrainingModelsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for TrainingModels.
     */
    cursor?: TrainingModelsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` TrainingModels from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` TrainingModels.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of TrainingModels.
     */
    distinct?: TrainingModelsScalarFieldEnum | TrainingModelsScalarFieldEnum[]
  }

  /**
   * TrainingModels findFirstOrThrow
   */
  export type TrainingModelsFindFirstOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
    /**
     * Filter, which TrainingModels to fetch.
     */
    where?: TrainingModelsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of TrainingModels to fetch.
     */
    orderBy?: TrainingModelsOrderByWithRelationInput | TrainingModelsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for TrainingModels.
     */
    cursor?: TrainingModelsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` TrainingModels from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` TrainingModels.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of TrainingModels.
     */
    distinct?: TrainingModelsScalarFieldEnum | TrainingModelsScalarFieldEnum[]
  }

  /**
   * TrainingModels findMany
   */
  export type TrainingModelsFindManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
    /**
     * Filter, which TrainingModels to fetch.
     */
    where?: TrainingModelsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of TrainingModels to fetch.
     */
    orderBy?: TrainingModelsOrderByWithRelationInput | TrainingModelsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for listing TrainingModels.
     */
    cursor?: TrainingModelsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` TrainingModels from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` TrainingModels.
     */
    skip?: number
    distinct?: TrainingModelsScalarFieldEnum | TrainingModelsScalarFieldEnum[]
  }

  /**
   * TrainingModels create
   */
  export type TrainingModelsCreateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
    /**
     * The data needed to create a TrainingModels.
     */
    data: XOR<TrainingModelsCreateInput, TrainingModelsUncheckedCreateInput>
  }

  /**
   * TrainingModels createMany
   */
  export type TrainingModelsCreateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to create many TrainingModels.
     */
    data: TrainingModelsCreateManyInput | TrainingModelsCreateManyInput[]
    skipDuplicates?: boolean
  }

  /**
   * TrainingModels createManyAndReturn
   */
  export type TrainingModelsCreateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelectCreateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * The data used to create many TrainingModels.
     */
    data: TrainingModelsCreateManyInput | TrainingModelsCreateManyInput[]
    skipDuplicates?: boolean
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsIncludeCreateManyAndReturn<ExtArgs> | null
  }

  /**
   * TrainingModels update
   */
  export type TrainingModelsUpdateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
    /**
     * The data needed to update a TrainingModels.
     */
    data: XOR<TrainingModelsUpdateInput, TrainingModelsUncheckedUpdateInput>
    /**
     * Choose, which TrainingModels to update.
     */
    where: TrainingModelsWhereUniqueInput
  }

  /**
   * TrainingModels updateMany
   */
  export type TrainingModelsUpdateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to update TrainingModels.
     */
    data: XOR<TrainingModelsUpdateManyMutationInput, TrainingModelsUncheckedUpdateManyInput>
    /**
     * Filter which TrainingModels to update
     */
    where?: TrainingModelsWhereInput
    /**
     * Limit how many TrainingModels to update.
     */
    limit?: number
  }

  /**
   * TrainingModels updateManyAndReturn
   */
  export type TrainingModelsUpdateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelectUpdateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * The data used to update TrainingModels.
     */
    data: XOR<TrainingModelsUpdateManyMutationInput, TrainingModelsUncheckedUpdateManyInput>
    /**
     * Filter which TrainingModels to update
     */
    where?: TrainingModelsWhereInput
    /**
     * Limit how many TrainingModels to update.
     */
    limit?: number
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsIncludeUpdateManyAndReturn<ExtArgs> | null
  }

  /**
   * TrainingModels upsert
   */
  export type TrainingModelsUpsertArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
    /**
     * The filter to search for the TrainingModels to update in case it exists.
     */
    where: TrainingModelsWhereUniqueInput
    /**
     * In case the TrainingModels found by the `where` argument doesn't exist, create a new TrainingModels with this data.
     */
    create: XOR<TrainingModelsCreateInput, TrainingModelsUncheckedCreateInput>
    /**
     * In case the TrainingModels was found with the provided `where` argument, update it with this data.
     */
    update: XOR<TrainingModelsUpdateInput, TrainingModelsUncheckedUpdateInput>
  }

  /**
   * TrainingModels delete
   */
  export type TrainingModelsDeleteArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
    /**
     * Filter which TrainingModels to delete.
     */
    where: TrainingModelsWhereUniqueInput
  }

  /**
   * TrainingModels deleteMany
   */
  export type TrainingModelsDeleteManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which TrainingModels to delete
     */
    where?: TrainingModelsWhereInput
    /**
     * Limit how many TrainingModels to delete.
     */
    limit?: number
  }

  /**
   * TrainingModels.Images
   */
  export type TrainingModels$ImagesArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    where?: ImagesWhereInput
    orderBy?: ImagesOrderByWithRelationInput | ImagesOrderByWithRelationInput[]
    cursor?: ImagesWhereUniqueInput
    take?: number
    skip?: number
    distinct?: ImagesScalarFieldEnum | ImagesScalarFieldEnum[]
  }

  /**
   * TrainingModels without action
   */
  export type TrainingModelsDefaultArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the TrainingModels
     */
    select?: TrainingModelsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the TrainingModels
     */
    omit?: TrainingModelsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TrainingModelsInclude<ExtArgs> | null
  }


  /**
   * Model UserCredits
   */

  export type AggregateUserCredits = {
    _count: UserCreditsCountAggregateOutputType | null
    _avg: UserCreditsAvgAggregateOutputType | null
    _sum: UserCreditsSumAggregateOutputType | null
    _min: UserCreditsMinAggregateOutputType | null
    _max: UserCreditsMaxAggregateOutputType | null
  }

  export type UserCreditsAvgAggregateOutputType = {
    id: number | null
    user_id: number | null
    credit_amount: number | null
    model_amount: number | null
  }

  export type UserCreditsSumAggregateOutputType = {
    id: number | null
    user_id: number | null
    credit_amount: number | null
    model_amount: number | null
  }

  export type UserCreditsMinAggregateOutputType = {
    id: number | null
    pid: string | null
    user_id: number | null
    credit_amount: number | null
    model_amount: number | null
    created_at: Date | null
    updated_at: Date | null
  }

  export type UserCreditsMaxAggregateOutputType = {
    id: number | null
    pid: string | null
    user_id: number | null
    credit_amount: number | null
    model_amount: number | null
    created_at: Date | null
    updated_at: Date | null
  }

  export type UserCreditsCountAggregateOutputType = {
    id: number
    pid: number
    user_id: number
    credit_amount: number
    model_amount: number
    created_at: number
    updated_at: number
    _all: number
  }


  export type UserCreditsAvgAggregateInputType = {
    id?: true
    user_id?: true
    credit_amount?: true
    model_amount?: true
  }

  export type UserCreditsSumAggregateInputType = {
    id?: true
    user_id?: true
    credit_amount?: true
    model_amount?: true
  }

  export type UserCreditsMinAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    credit_amount?: true
    model_amount?: true
    created_at?: true
    updated_at?: true
  }

  export type UserCreditsMaxAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    credit_amount?: true
    model_amount?: true
    created_at?: true
    updated_at?: true
  }

  export type UserCreditsCountAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    credit_amount?: true
    model_amount?: true
    created_at?: true
    updated_at?: true
    _all?: true
  }

  export type UserCreditsAggregateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which UserCredits to aggregate.
     */
    where?: UserCreditsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of UserCredits to fetch.
     */
    orderBy?: UserCreditsOrderByWithRelationInput | UserCreditsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the start position
     */
    cursor?: UserCreditsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` UserCredits from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` UserCredits.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Count returned UserCredits
    **/
    _count?: true | UserCreditsCountAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to average
    **/
    _avg?: UserCreditsAvgAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to sum
    **/
    _sum?: UserCreditsSumAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the minimum value
    **/
    _min?: UserCreditsMinAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the maximum value
    **/
    _max?: UserCreditsMaxAggregateInputType
  }

  export type GetUserCreditsAggregateType<T extends UserCreditsAggregateArgs> = {
        [P in keyof T & keyof AggregateUserCredits]: P extends '_count' | 'count'
      ? T[P] extends true
        ? number
        : GetScalarType<T[P], AggregateUserCredits[P]>
      : GetScalarType<T[P], AggregateUserCredits[P]>
  }




  export type UserCreditsGroupByArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: UserCreditsWhereInput
    orderBy?: UserCreditsOrderByWithAggregationInput | UserCreditsOrderByWithAggregationInput[]
    by: UserCreditsScalarFieldEnum[] | UserCreditsScalarFieldEnum
    having?: UserCreditsScalarWhereWithAggregatesInput
    take?: number
    skip?: number
    _count?: UserCreditsCountAggregateInputType | true
    _avg?: UserCreditsAvgAggregateInputType
    _sum?: UserCreditsSumAggregateInputType
    _min?: UserCreditsMinAggregateInputType
    _max?: UserCreditsMaxAggregateInputType
  }

  export type UserCreditsGroupByOutputType = {
    id: number
    pid: string
    user_id: number
    credit_amount: number
    model_amount: number
    created_at: Date
    updated_at: Date
    _count: UserCreditsCountAggregateOutputType | null
    _avg: UserCreditsAvgAggregateOutputType | null
    _sum: UserCreditsSumAggregateOutputType | null
    _min: UserCreditsMinAggregateOutputType | null
    _max: UserCreditsMaxAggregateOutputType | null
  }

  type GetUserCreditsGroupByPayload<T extends UserCreditsGroupByArgs> = Prisma.PrismaPromise<
    Array<
      PickEnumerable<UserCreditsGroupByOutputType, T['by']> &
        {
          [P in ((keyof T) & (keyof UserCreditsGroupByOutputType))]: P extends '_count'
            ? T[P] extends boolean
              ? number
              : GetScalarType<T[P], UserCreditsGroupByOutputType[P]>
            : GetScalarType<T[P], UserCreditsGroupByOutputType[P]>
        }
      >
    >


  export type UserCreditsSelect<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    credit_amount?: boolean
    model_amount?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["userCredits"]>

  export type UserCreditsSelectCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    credit_amount?: boolean
    model_amount?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["userCredits"]>

  export type UserCreditsSelectUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    credit_amount?: boolean
    model_amount?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["userCredits"]>

  export type UserCreditsSelectScalar = {
    id?: boolean
    pid?: boolean
    user_id?: boolean
    credit_amount?: boolean
    model_amount?: boolean
    created_at?: boolean
    updated_at?: boolean
  }

  export type UserCreditsOmit<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetOmit<"id" | "pid" | "user_id" | "credit_amount" | "model_amount" | "created_at" | "updated_at", ExtArgs["result"]["userCredits"]>
  export type UserCreditsInclude<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }
  export type UserCreditsIncludeCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }
  export type UserCreditsIncludeUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }

  export type $UserCreditsPayload<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    name: "UserCredits"
    objects: {
      user: Prisma.$UsersPayload<ExtArgs>
    }
    scalars: $Extensions.GetPayloadResult<{
      id: number
      pid: string
      user_id: number
      credit_amount: number
      model_amount: number
      created_at: Date
      updated_at: Date
    }, ExtArgs["result"]["userCredits"]>
    composites: {}
  }

  type UserCreditsGetPayload<S extends boolean | null | undefined | UserCreditsDefaultArgs> = $Result.GetResult<Prisma.$UserCreditsPayload, S>

  type UserCreditsCountArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> =
    Omit<UserCreditsFindManyArgs, 'select' | 'include' | 'distinct' | 'omit'> & {
      select?: UserCreditsCountAggregateInputType | true
    }

  export interface UserCreditsDelegate<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> {
    [K: symbol]: { types: Prisma.TypeMap<ExtArgs>['model']['UserCredits'], meta: { name: 'UserCredits' } }
    /**
     * Find zero or one UserCredits that matches the filter.
     * @param {UserCreditsFindUniqueArgs} args - Arguments to find a UserCredits
     * @example
     * // Get one UserCredits
     * const userCredits = await prisma.userCredits.findUnique({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUnique<T extends UserCreditsFindUniqueArgs>(args: SelectSubset<T, UserCreditsFindUniqueArgs<ExtArgs>>): Prisma__UserCreditsClient<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "findUnique", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find one UserCredits that matches the filter or throw an error with `error.code='P2025'`
     * if no matches were found.
     * @param {UserCreditsFindUniqueOrThrowArgs} args - Arguments to find a UserCredits
     * @example
     * // Get one UserCredits
     * const userCredits = await prisma.userCredits.findUniqueOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUniqueOrThrow<T extends UserCreditsFindUniqueOrThrowArgs>(args: SelectSubset<T, UserCreditsFindUniqueOrThrowArgs<ExtArgs>>): Prisma__UserCreditsClient<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first UserCredits that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UserCreditsFindFirstArgs} args - Arguments to find a UserCredits
     * @example
     * // Get one UserCredits
     * const userCredits = await prisma.userCredits.findFirst({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirst<T extends UserCreditsFindFirstArgs>(args?: SelectSubset<T, UserCreditsFindFirstArgs<ExtArgs>>): Prisma__UserCreditsClient<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "findFirst", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first UserCredits that matches the filter or
     * throw `PrismaKnownClientError` with `P2025` code if no matches were found.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UserCreditsFindFirstOrThrowArgs} args - Arguments to find a UserCredits
     * @example
     * // Get one UserCredits
     * const userCredits = await prisma.userCredits.findFirstOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirstOrThrow<T extends UserCreditsFindFirstOrThrowArgs>(args?: SelectSubset<T, UserCreditsFindFirstOrThrowArgs<ExtArgs>>): Prisma__UserCreditsClient<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "findFirstOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find zero or more UserCredits that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UserCreditsFindManyArgs} args - Arguments to filter and select certain fields only.
     * @example
     * // Get all UserCredits
     * const userCredits = await prisma.userCredits.findMany()
     * 
     * // Get first 10 UserCredits
     * const userCredits = await prisma.userCredits.findMany({ take: 10 })
     * 
     * // Only select the `id`
     * const userCreditsWithIdOnly = await prisma.userCredits.findMany({ select: { id: true } })
     * 
     */
    findMany<T extends UserCreditsFindManyArgs>(args?: SelectSubset<T, UserCreditsFindManyArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "findMany", GlobalOmitOptions>>

    /**
     * Create a UserCredits.
     * @param {UserCreditsCreateArgs} args - Arguments to create a UserCredits.
     * @example
     * // Create one UserCredits
     * const UserCredits = await prisma.userCredits.create({
     *   data: {
     *     // ... data to create a UserCredits
     *   }
     * })
     * 
     */
    create<T extends UserCreditsCreateArgs>(args: SelectSubset<T, UserCreditsCreateArgs<ExtArgs>>): Prisma__UserCreditsClient<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "create", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Create many UserCredits.
     * @param {UserCreditsCreateManyArgs} args - Arguments to create many UserCredits.
     * @example
     * // Create many UserCredits
     * const userCredits = await prisma.userCredits.createMany({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     *     
     */
    createMany<T extends UserCreditsCreateManyArgs>(args?: SelectSubset<T, UserCreditsCreateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Create many UserCredits and returns the data saved in the database.
     * @param {UserCreditsCreateManyAndReturnArgs} args - Arguments to create many UserCredits.
     * @example
     * // Create many UserCredits
     * const userCredits = await prisma.userCredits.createManyAndReturn({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Create many UserCredits and only return the `id`
     * const userCreditsWithIdOnly = await prisma.userCredits.createManyAndReturn({
     *   select: { id: true },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    createManyAndReturn<T extends UserCreditsCreateManyAndReturnArgs>(args?: SelectSubset<T, UserCreditsCreateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "createManyAndReturn", GlobalOmitOptions>>

    /**
     * Delete a UserCredits.
     * @param {UserCreditsDeleteArgs} args - Arguments to delete one UserCredits.
     * @example
     * // Delete one UserCredits
     * const UserCredits = await prisma.userCredits.delete({
     *   where: {
     *     // ... filter to delete one UserCredits
     *   }
     * })
     * 
     */
    delete<T extends UserCreditsDeleteArgs>(args: SelectSubset<T, UserCreditsDeleteArgs<ExtArgs>>): Prisma__UserCreditsClient<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "delete", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Update one UserCredits.
     * @param {UserCreditsUpdateArgs} args - Arguments to update one UserCredits.
     * @example
     * // Update one UserCredits
     * const userCredits = await prisma.userCredits.update({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    update<T extends UserCreditsUpdateArgs>(args: SelectSubset<T, UserCreditsUpdateArgs<ExtArgs>>): Prisma__UserCreditsClient<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "update", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Delete zero or more UserCredits.
     * @param {UserCreditsDeleteManyArgs} args - Arguments to filter UserCredits to delete.
     * @example
     * // Delete a few UserCredits
     * const { count } = await prisma.userCredits.deleteMany({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     * 
     */
    deleteMany<T extends UserCreditsDeleteManyArgs>(args?: SelectSubset<T, UserCreditsDeleteManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more UserCredits.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UserCreditsUpdateManyArgs} args - Arguments to update one or more rows.
     * @example
     * // Update many UserCredits
     * const userCredits = await prisma.userCredits.updateMany({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    updateMany<T extends UserCreditsUpdateManyArgs>(args: SelectSubset<T, UserCreditsUpdateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more UserCredits and returns the data updated in the database.
     * @param {UserCreditsUpdateManyAndReturnArgs} args - Arguments to update many UserCredits.
     * @example
     * // Update many UserCredits
     * const userCredits = await prisma.userCredits.updateManyAndReturn({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Update zero or more UserCredits and only return the `id`
     * const userCreditsWithIdOnly = await prisma.userCredits.updateManyAndReturn({
     *   select: { id: true },
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    updateManyAndReturn<T extends UserCreditsUpdateManyAndReturnArgs>(args: SelectSubset<T, UserCreditsUpdateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "updateManyAndReturn", GlobalOmitOptions>>

    /**
     * Create or update one UserCredits.
     * @param {UserCreditsUpsertArgs} args - Arguments to update or create a UserCredits.
     * @example
     * // Update or create a UserCredits
     * const userCredits = await prisma.userCredits.upsert({
     *   create: {
     *     // ... data to create a UserCredits
     *   },
     *   update: {
     *     // ... in case it already exists, update
     *   },
     *   where: {
     *     // ... the filter for the UserCredits we want to update
     *   }
     * })
     */
    upsert<T extends UserCreditsUpsertArgs>(args: SelectSubset<T, UserCreditsUpsertArgs<ExtArgs>>): Prisma__UserCreditsClient<$Result.GetResult<Prisma.$UserCreditsPayload<ExtArgs>, T, "upsert", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>


    /**
     * Count the number of UserCredits.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UserCreditsCountArgs} args - Arguments to filter UserCredits to count.
     * @example
     * // Count the number of UserCredits
     * const count = await prisma.userCredits.count({
     *   where: {
     *     // ... the filter for the UserCredits we want to count
     *   }
     * })
    **/
    count<T extends UserCreditsCountArgs>(
      args?: Subset<T, UserCreditsCountArgs>,
    ): Prisma.PrismaPromise<
      T extends $Utils.Record<'select', any>
        ? T['select'] extends true
          ? number
          : GetScalarType<T['select'], UserCreditsCountAggregateOutputType>
        : number
    >

    /**
     * Allows you to perform aggregations operations on a UserCredits.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UserCreditsAggregateArgs} args - Select which aggregations you would like to apply and on what fields.
     * @example
     * // Ordered by age ascending
     * // Where email contains prisma.io
     * // Limited to the 10 users
     * const aggregations = await prisma.user.aggregate({
     *   _avg: {
     *     age: true,
     *   },
     *   where: {
     *     email: {
     *       contains: "prisma.io",
     *     },
     *   },
     *   orderBy: {
     *     age: "asc",
     *   },
     *   take: 10,
     * })
    **/
    aggregate<T extends UserCreditsAggregateArgs>(args: Subset<T, UserCreditsAggregateArgs>): Prisma.PrismaPromise<GetUserCreditsAggregateType<T>>

    /**
     * Group by UserCredits.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {UserCreditsGroupByArgs} args - Group by arguments.
     * @example
     * // Group by city, order by createdAt, get count
     * const result = await prisma.user.groupBy({
     *   by: ['city', 'createdAt'],
     *   orderBy: {
     *     createdAt: true
     *   },
     *   _count: {
     *     _all: true
     *   },
     * })
     * 
    **/
    groupBy<
      T extends UserCreditsGroupByArgs,
      HasSelectOrTake extends Or<
        Extends<'skip', Keys<T>>,
        Extends<'take', Keys<T>>
      >,
      OrderByArg extends True extends HasSelectOrTake
        ? { orderBy: UserCreditsGroupByArgs['orderBy'] }
        : { orderBy?: UserCreditsGroupByArgs['orderBy'] },
      OrderFields extends ExcludeUnderscoreKeys<Keys<MaybeTupleToUnion<T['orderBy']>>>,
      ByFields extends MaybeTupleToUnion<T['by']>,
      ByValid extends Has<ByFields, OrderFields>,
      HavingFields extends GetHavingFields<T['having']>,
      HavingValid extends Has<ByFields, HavingFields>,
      ByEmpty extends T['by'] extends never[] ? True : False,
      InputErrors extends ByEmpty extends True
      ? `Error: "by" must not be empty.`
      : HavingValid extends False
      ? {
          [P in HavingFields]: P extends ByFields
            ? never
            : P extends string
            ? `Error: Field "${P}" used in "having" needs to be provided in "by".`
            : [
                Error,
                'Field ',
                P,
                ` in "having" needs to be provided in "by"`,
              ]
        }[HavingFields]
      : 'take' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "take", you also need to provide "orderBy"'
      : 'skip' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "skip", you also need to provide "orderBy"'
      : ByValid extends True
      ? {}
      : {
          [P in OrderFields]: P extends ByFields
            ? never
            : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
        }[OrderFields]
    >(args: SubsetIntersection<T, UserCreditsGroupByArgs, OrderByArg> & InputErrors): {} extends InputErrors ? GetUserCreditsGroupByPayload<T> : Prisma.PrismaPromise<InputErrors>
  /**
   * Fields of the UserCredits model
   */
  readonly fields: UserCreditsFieldRefs;
  }

  /**
   * The delegate class that acts as a "Promise-like" for UserCredits.
   * Why is this prefixed with `Prisma__`?
   * Because we want to prevent naming conflicts as mentioned in
   * https://github.com/prisma/prisma-client-js/issues/707
   */
  export interface Prisma__UserCreditsClient<T, Null = never, ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> extends Prisma.PrismaPromise<T> {
    readonly [Symbol.toStringTag]: "PrismaPromise"
    user<T extends UsersDefaultArgs<ExtArgs> = {}>(args?: Subset<T, UsersDefaultArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions> | Null, Null, ExtArgs, GlobalOmitOptions>
    /**
     * Attaches callbacks for the resolution and/or rejection of the Promise.
     * @param onfulfilled The callback to execute when the Promise is resolved.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of which ever callback is executed.
     */
    then<TResult1 = T, TResult2 = never>(onfulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): $Utils.JsPromise<TResult1 | TResult2>
    /**
     * Attaches a callback for only the rejection of the Promise.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of the callback.
     */
    catch<TResult = never>(onrejected?: ((reason: any) => TResult | PromiseLike<TResult>) | undefined | null): $Utils.JsPromise<T | TResult>
    /**
     * Attaches a callback that is invoked when the Promise is settled (fulfilled or rejected). The
     * resolved value cannot be modified from the callback.
     * @param onfinally The callback to execute when the Promise is settled (fulfilled or rejected).
     * @returns A Promise for the completion of the callback.
     */
    finally(onfinally?: (() => void) | undefined | null): $Utils.JsPromise<T>
  }




  /**
   * Fields of the UserCredits model
   */ 
  interface UserCreditsFieldRefs {
    readonly id: FieldRef<"UserCredits", 'Int'>
    readonly pid: FieldRef<"UserCredits", 'String'>
    readonly user_id: FieldRef<"UserCredits", 'Int'>
    readonly credit_amount: FieldRef<"UserCredits", 'Int'>
    readonly model_amount: FieldRef<"UserCredits", 'Int'>
    readonly created_at: FieldRef<"UserCredits", 'DateTime'>
    readonly updated_at: FieldRef<"UserCredits", 'DateTime'>
  }
    

  // Custom InputTypes
  /**
   * UserCredits findUnique
   */
  export type UserCreditsFindUniqueArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
    /**
     * Filter, which UserCredits to fetch.
     */
    where: UserCreditsWhereUniqueInput
  }

  /**
   * UserCredits findUniqueOrThrow
   */
  export type UserCreditsFindUniqueOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
    /**
     * Filter, which UserCredits to fetch.
     */
    where: UserCreditsWhereUniqueInput
  }

  /**
   * UserCredits findFirst
   */
  export type UserCreditsFindFirstArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
    /**
     * Filter, which UserCredits to fetch.
     */
    where?: UserCreditsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of UserCredits to fetch.
     */
    orderBy?: UserCreditsOrderByWithRelationInput | UserCreditsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for UserCredits.
     */
    cursor?: UserCreditsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` UserCredits from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` UserCredits.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of UserCredits.
     */
    distinct?: UserCreditsScalarFieldEnum | UserCreditsScalarFieldEnum[]
  }

  /**
   * UserCredits findFirstOrThrow
   */
  export type UserCreditsFindFirstOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
    /**
     * Filter, which UserCredits to fetch.
     */
    where?: UserCreditsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of UserCredits to fetch.
     */
    orderBy?: UserCreditsOrderByWithRelationInput | UserCreditsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for UserCredits.
     */
    cursor?: UserCreditsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` UserCredits from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` UserCredits.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of UserCredits.
     */
    distinct?: UserCreditsScalarFieldEnum | UserCreditsScalarFieldEnum[]
  }

  /**
   * UserCredits findMany
   */
  export type UserCreditsFindManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
    /**
     * Filter, which UserCredits to fetch.
     */
    where?: UserCreditsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of UserCredits to fetch.
     */
    orderBy?: UserCreditsOrderByWithRelationInput | UserCreditsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for listing UserCredits.
     */
    cursor?: UserCreditsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` UserCredits from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` UserCredits.
     */
    skip?: number
    distinct?: UserCreditsScalarFieldEnum | UserCreditsScalarFieldEnum[]
  }

  /**
   * UserCredits create
   */
  export type UserCreditsCreateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
    /**
     * The data needed to create a UserCredits.
     */
    data: XOR<UserCreditsCreateInput, UserCreditsUncheckedCreateInput>
  }

  /**
   * UserCredits createMany
   */
  export type UserCreditsCreateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to create many UserCredits.
     */
    data: UserCreditsCreateManyInput | UserCreditsCreateManyInput[]
    skipDuplicates?: boolean
  }

  /**
   * UserCredits createManyAndReturn
   */
  export type UserCreditsCreateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelectCreateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * The data used to create many UserCredits.
     */
    data: UserCreditsCreateManyInput | UserCreditsCreateManyInput[]
    skipDuplicates?: boolean
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsIncludeCreateManyAndReturn<ExtArgs> | null
  }

  /**
   * UserCredits update
   */
  export type UserCreditsUpdateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
    /**
     * The data needed to update a UserCredits.
     */
    data: XOR<UserCreditsUpdateInput, UserCreditsUncheckedUpdateInput>
    /**
     * Choose, which UserCredits to update.
     */
    where: UserCreditsWhereUniqueInput
  }

  /**
   * UserCredits updateMany
   */
  export type UserCreditsUpdateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to update UserCredits.
     */
    data: XOR<UserCreditsUpdateManyMutationInput, UserCreditsUncheckedUpdateManyInput>
    /**
     * Filter which UserCredits to update
     */
    where?: UserCreditsWhereInput
    /**
     * Limit how many UserCredits to update.
     */
    limit?: number
  }

  /**
   * UserCredits updateManyAndReturn
   */
  export type UserCreditsUpdateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelectUpdateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * The data used to update UserCredits.
     */
    data: XOR<UserCreditsUpdateManyMutationInput, UserCreditsUncheckedUpdateManyInput>
    /**
     * Filter which UserCredits to update
     */
    where?: UserCreditsWhereInput
    /**
     * Limit how many UserCredits to update.
     */
    limit?: number
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsIncludeUpdateManyAndReturn<ExtArgs> | null
  }

  /**
   * UserCredits upsert
   */
  export type UserCreditsUpsertArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
    /**
     * The filter to search for the UserCredits to update in case it exists.
     */
    where: UserCreditsWhereUniqueInput
    /**
     * In case the UserCredits found by the `where` argument doesn't exist, create a new UserCredits with this data.
     */
    create: XOR<UserCreditsCreateInput, UserCreditsUncheckedCreateInput>
    /**
     * In case the UserCredits was found with the provided `where` argument, update it with this data.
     */
    update: XOR<UserCreditsUpdateInput, UserCreditsUncheckedUpdateInput>
  }

  /**
   * UserCredits delete
   */
  export type UserCreditsDeleteArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
    /**
     * Filter which UserCredits to delete.
     */
    where: UserCreditsWhereUniqueInput
  }

  /**
   * UserCredits deleteMany
   */
  export type UserCreditsDeleteManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which UserCredits to delete
     */
    where?: UserCreditsWhereInput
    /**
     * Limit how many UserCredits to delete.
     */
    limit?: number
  }

  /**
   * UserCredits without action
   */
  export type UserCreditsDefaultArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the UserCredits
     */
    select?: UserCreditsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the UserCredits
     */
    omit?: UserCreditsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: UserCreditsInclude<ExtArgs> | null
  }


  /**
   * Model Images
   */

  export type AggregateImages = {
    _count: ImagesCountAggregateOutputType | null
    _avg: ImagesAvgAggregateOutputType | null
    _sum: ImagesSumAggregateOutputType | null
    _min: ImagesMinAggregateOutputType | null
    _max: ImagesMaxAggregateOutputType | null
  }

  export type ImagesAvgAggregateOutputType = {
    id: number | null
    user_id: number | null
    training_model_id: number | null
    pack_id: number | null
    num_inference_steps: number | null
    width: number | null
    height: number | null
  }

  export type ImagesSumAggregateOutputType = {
    id: number | null
    user_id: number | null
    training_model_id: number | null
    pack_id: number | null
    num_inference_steps: number | null
    width: number | null
    height: number | null
  }

  export type ImagesMinAggregateOutputType = {
    id: number | null
    pid: string | null
    user_id: number | null
    training_model_id: number | null
    pack_id: number | null
    user_prompt: string | null
    sys_prompt: string | null
    alt: string | null
    num_inference_steps: number | null
    content_type: $Enums.ImageFormat | null
    status: $Enums.Status | null
    image_size: $Enums.ImageSize | null
    fal_ai_request_id: string | null
    width: number | null
    height: number | null
    image_url: string | null
    image_url_s3: string | null
    is_favorite: boolean | null
    deleted_at: Date | null
    created_at: Date | null
    updated_at: Date | null
  }

  export type ImagesMaxAggregateOutputType = {
    id: number | null
    pid: string | null
    user_id: number | null
    training_model_id: number | null
    pack_id: number | null
    user_prompt: string | null
    sys_prompt: string | null
    alt: string | null
    num_inference_steps: number | null
    content_type: $Enums.ImageFormat | null
    status: $Enums.Status | null
    image_size: $Enums.ImageSize | null
    fal_ai_request_id: string | null
    width: number | null
    height: number | null
    image_url: string | null
    image_url_s3: string | null
    is_favorite: boolean | null
    deleted_at: Date | null
    created_at: Date | null
    updated_at: Date | null
  }

  export type ImagesCountAggregateOutputType = {
    id: number
    pid: number
    user_id: number
    training_model_id: number
    pack_id: number
    user_prompt: number
    sys_prompt: number
    alt: number
    num_inference_steps: number
    content_type: number
    status: number
    image_size: number
    fal_ai_request_id: number
    width: number
    height: number
    image_url: number
    image_url_s3: number
    is_favorite: number
    deleted_at: number
    created_at: number
    updated_at: number
    _all: number
  }


  export type ImagesAvgAggregateInputType = {
    id?: true
    user_id?: true
    training_model_id?: true
    pack_id?: true
    num_inference_steps?: true
    width?: true
    height?: true
  }

  export type ImagesSumAggregateInputType = {
    id?: true
    user_id?: true
    training_model_id?: true
    pack_id?: true
    num_inference_steps?: true
    width?: true
    height?: true
  }

  export type ImagesMinAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    training_model_id?: true
    pack_id?: true
    user_prompt?: true
    sys_prompt?: true
    alt?: true
    num_inference_steps?: true
    content_type?: true
    status?: true
    image_size?: true
    fal_ai_request_id?: true
    width?: true
    height?: true
    image_url?: true
    image_url_s3?: true
    is_favorite?: true
    deleted_at?: true
    created_at?: true
    updated_at?: true
  }

  export type ImagesMaxAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    training_model_id?: true
    pack_id?: true
    user_prompt?: true
    sys_prompt?: true
    alt?: true
    num_inference_steps?: true
    content_type?: true
    status?: true
    image_size?: true
    fal_ai_request_id?: true
    width?: true
    height?: true
    image_url?: true
    image_url_s3?: true
    is_favorite?: true
    deleted_at?: true
    created_at?: true
    updated_at?: true
  }

  export type ImagesCountAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    training_model_id?: true
    pack_id?: true
    user_prompt?: true
    sys_prompt?: true
    alt?: true
    num_inference_steps?: true
    content_type?: true
    status?: true
    image_size?: true
    fal_ai_request_id?: true
    width?: true
    height?: true
    image_url?: true
    image_url_s3?: true
    is_favorite?: true
    deleted_at?: true
    created_at?: true
    updated_at?: true
    _all?: true
  }

  export type ImagesAggregateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which Images to aggregate.
     */
    where?: ImagesWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Images to fetch.
     */
    orderBy?: ImagesOrderByWithRelationInput | ImagesOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the start position
     */
    cursor?: ImagesWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Images from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Images.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Count returned Images
    **/
    _count?: true | ImagesCountAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to average
    **/
    _avg?: ImagesAvgAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to sum
    **/
    _sum?: ImagesSumAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the minimum value
    **/
    _min?: ImagesMinAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the maximum value
    **/
    _max?: ImagesMaxAggregateInputType
  }

  export type GetImagesAggregateType<T extends ImagesAggregateArgs> = {
        [P in keyof T & keyof AggregateImages]: P extends '_count' | 'count'
      ? T[P] extends true
        ? number
        : GetScalarType<T[P], AggregateImages[P]>
      : GetScalarType<T[P], AggregateImages[P]>
  }




  export type ImagesGroupByArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: ImagesWhereInput
    orderBy?: ImagesOrderByWithAggregationInput | ImagesOrderByWithAggregationInput[]
    by: ImagesScalarFieldEnum[] | ImagesScalarFieldEnum
    having?: ImagesScalarWhereWithAggregatesInput
    take?: number
    skip?: number
    _count?: ImagesCountAggregateInputType | true
    _avg?: ImagesAvgAggregateInputType
    _sum?: ImagesSumAggregateInputType
    _min?: ImagesMinAggregateInputType
    _max?: ImagesMaxAggregateInputType
  }

  export type ImagesGroupByOutputType = {
    id: number
    pid: string
    user_id: number
    training_model_id: number
    pack_id: number | null
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id: string | null
    width: number | null
    height: number | null
    image_url: string | null
    image_url_s3: string | null
    is_favorite: boolean
    deleted_at: Date | null
    created_at: Date
    updated_at: Date
    _count: ImagesCountAggregateOutputType | null
    _avg: ImagesAvgAggregateOutputType | null
    _sum: ImagesSumAggregateOutputType | null
    _min: ImagesMinAggregateOutputType | null
    _max: ImagesMaxAggregateOutputType | null
  }

  type GetImagesGroupByPayload<T extends ImagesGroupByArgs> = Prisma.PrismaPromise<
    Array<
      PickEnumerable<ImagesGroupByOutputType, T['by']> &
        {
          [P in ((keyof T) & (keyof ImagesGroupByOutputType))]: P extends '_count'
            ? T[P] extends boolean
              ? number
              : GetScalarType<T[P], ImagesGroupByOutputType[P]>
            : GetScalarType<T[P], ImagesGroupByOutputType[P]>
        }
      >
    >


  export type ImagesSelect<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    training_model_id?: boolean
    pack_id?: boolean
    user_prompt?: boolean
    sys_prompt?: boolean
    alt?: boolean
    num_inference_steps?: boolean
    content_type?: boolean
    status?: boolean
    image_size?: boolean
    fal_ai_request_id?: boolean
    width?: boolean
    height?: boolean
    image_url?: boolean
    image_url_s3?: boolean
    is_favorite?: boolean
    deleted_at?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
    training_model?: boolean | TrainingModelsDefaultArgs<ExtArgs>
    pack?: boolean | Images$packArgs<ExtArgs>
  }, ExtArgs["result"]["images"]>

  export type ImagesSelectCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    training_model_id?: boolean
    pack_id?: boolean
    user_prompt?: boolean
    sys_prompt?: boolean
    alt?: boolean
    num_inference_steps?: boolean
    content_type?: boolean
    status?: boolean
    image_size?: boolean
    fal_ai_request_id?: boolean
    width?: boolean
    height?: boolean
    image_url?: boolean
    image_url_s3?: boolean
    is_favorite?: boolean
    deleted_at?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
    training_model?: boolean | TrainingModelsDefaultArgs<ExtArgs>
    pack?: boolean | Images$packArgs<ExtArgs>
  }, ExtArgs["result"]["images"]>

  export type ImagesSelectUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    training_model_id?: boolean
    pack_id?: boolean
    user_prompt?: boolean
    sys_prompt?: boolean
    alt?: boolean
    num_inference_steps?: boolean
    content_type?: boolean
    status?: boolean
    image_size?: boolean
    fal_ai_request_id?: boolean
    width?: boolean
    height?: boolean
    image_url?: boolean
    image_url_s3?: boolean
    is_favorite?: boolean
    deleted_at?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
    training_model?: boolean | TrainingModelsDefaultArgs<ExtArgs>
    pack?: boolean | Images$packArgs<ExtArgs>
  }, ExtArgs["result"]["images"]>

  export type ImagesSelectScalar = {
    id?: boolean
    pid?: boolean
    user_id?: boolean
    training_model_id?: boolean
    pack_id?: boolean
    user_prompt?: boolean
    sys_prompt?: boolean
    alt?: boolean
    num_inference_steps?: boolean
    content_type?: boolean
    status?: boolean
    image_size?: boolean
    fal_ai_request_id?: boolean
    width?: boolean
    height?: boolean
    image_url?: boolean
    image_url_s3?: boolean
    is_favorite?: boolean
    deleted_at?: boolean
    created_at?: boolean
    updated_at?: boolean
  }

  export type ImagesOmit<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetOmit<"id" | "pid" | "user_id" | "training_model_id" | "pack_id" | "user_prompt" | "sys_prompt" | "alt" | "num_inference_steps" | "content_type" | "status" | "image_size" | "fal_ai_request_id" | "width" | "height" | "image_url" | "image_url_s3" | "is_favorite" | "deleted_at" | "created_at" | "updated_at", ExtArgs["result"]["images"]>
  export type ImagesInclude<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
    training_model?: boolean | TrainingModelsDefaultArgs<ExtArgs>
    pack?: boolean | Images$packArgs<ExtArgs>
  }
  export type ImagesIncludeCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
    training_model?: boolean | TrainingModelsDefaultArgs<ExtArgs>
    pack?: boolean | Images$packArgs<ExtArgs>
  }
  export type ImagesIncludeUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
    training_model?: boolean | TrainingModelsDefaultArgs<ExtArgs>
    pack?: boolean | Images$packArgs<ExtArgs>
  }

  export type $ImagesPayload<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    name: "Images"
    objects: {
      user: Prisma.$UsersPayload<ExtArgs>
      training_model: Prisma.$TrainingModelsPayload<ExtArgs>
      pack: Prisma.$PacksPayload<ExtArgs> | null
    }
    scalars: $Extensions.GetPayloadResult<{
      id: number
      pid: string
      user_id: number
      training_model_id: number
      pack_id: number | null
      user_prompt: string
      sys_prompt: string
      alt: string
      num_inference_steps: number
      content_type: $Enums.ImageFormat
      status: $Enums.Status
      image_size: $Enums.ImageSize
      fal_ai_request_id: string | null
      width: number | null
      height: number | null
      image_url: string | null
      image_url_s3: string | null
      is_favorite: boolean
      deleted_at: Date | null
      created_at: Date
      updated_at: Date
    }, ExtArgs["result"]["images"]>
    composites: {}
  }

  type ImagesGetPayload<S extends boolean | null | undefined | ImagesDefaultArgs> = $Result.GetResult<Prisma.$ImagesPayload, S>

  type ImagesCountArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> =
    Omit<ImagesFindManyArgs, 'select' | 'include' | 'distinct' | 'omit'> & {
      select?: ImagesCountAggregateInputType | true
    }

  export interface ImagesDelegate<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> {
    [K: symbol]: { types: Prisma.TypeMap<ExtArgs>['model']['Images'], meta: { name: 'Images' } }
    /**
     * Find zero or one Images that matches the filter.
     * @param {ImagesFindUniqueArgs} args - Arguments to find a Images
     * @example
     * // Get one Images
     * const images = await prisma.images.findUnique({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUnique<T extends ImagesFindUniqueArgs>(args: SelectSubset<T, ImagesFindUniqueArgs<ExtArgs>>): Prisma__ImagesClient<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "findUnique", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find one Images that matches the filter or throw an error with `error.code='P2025'`
     * if no matches were found.
     * @param {ImagesFindUniqueOrThrowArgs} args - Arguments to find a Images
     * @example
     * // Get one Images
     * const images = await prisma.images.findUniqueOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUniqueOrThrow<T extends ImagesFindUniqueOrThrowArgs>(args: SelectSubset<T, ImagesFindUniqueOrThrowArgs<ExtArgs>>): Prisma__ImagesClient<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first Images that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {ImagesFindFirstArgs} args - Arguments to find a Images
     * @example
     * // Get one Images
     * const images = await prisma.images.findFirst({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirst<T extends ImagesFindFirstArgs>(args?: SelectSubset<T, ImagesFindFirstArgs<ExtArgs>>): Prisma__ImagesClient<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "findFirst", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first Images that matches the filter or
     * throw `PrismaKnownClientError` with `P2025` code if no matches were found.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {ImagesFindFirstOrThrowArgs} args - Arguments to find a Images
     * @example
     * // Get one Images
     * const images = await prisma.images.findFirstOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirstOrThrow<T extends ImagesFindFirstOrThrowArgs>(args?: SelectSubset<T, ImagesFindFirstOrThrowArgs<ExtArgs>>): Prisma__ImagesClient<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "findFirstOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find zero or more Images that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {ImagesFindManyArgs} args - Arguments to filter and select certain fields only.
     * @example
     * // Get all Images
     * const images = await prisma.images.findMany()
     * 
     * // Get first 10 Images
     * const images = await prisma.images.findMany({ take: 10 })
     * 
     * // Only select the `id`
     * const imagesWithIdOnly = await prisma.images.findMany({ select: { id: true } })
     * 
     */
    findMany<T extends ImagesFindManyArgs>(args?: SelectSubset<T, ImagesFindManyArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "findMany", GlobalOmitOptions>>

    /**
     * Create a Images.
     * @param {ImagesCreateArgs} args - Arguments to create a Images.
     * @example
     * // Create one Images
     * const Images = await prisma.images.create({
     *   data: {
     *     // ... data to create a Images
     *   }
     * })
     * 
     */
    create<T extends ImagesCreateArgs>(args: SelectSubset<T, ImagesCreateArgs<ExtArgs>>): Prisma__ImagesClient<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "create", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Create many Images.
     * @param {ImagesCreateManyArgs} args - Arguments to create many Images.
     * @example
     * // Create many Images
     * const images = await prisma.images.createMany({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     *     
     */
    createMany<T extends ImagesCreateManyArgs>(args?: SelectSubset<T, ImagesCreateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Create many Images and returns the data saved in the database.
     * @param {ImagesCreateManyAndReturnArgs} args - Arguments to create many Images.
     * @example
     * // Create many Images
     * const images = await prisma.images.createManyAndReturn({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Create many Images and only return the `id`
     * const imagesWithIdOnly = await prisma.images.createManyAndReturn({
     *   select: { id: true },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    createManyAndReturn<T extends ImagesCreateManyAndReturnArgs>(args?: SelectSubset<T, ImagesCreateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "createManyAndReturn", GlobalOmitOptions>>

    /**
     * Delete a Images.
     * @param {ImagesDeleteArgs} args - Arguments to delete one Images.
     * @example
     * // Delete one Images
     * const Images = await prisma.images.delete({
     *   where: {
     *     // ... filter to delete one Images
     *   }
     * })
     * 
     */
    delete<T extends ImagesDeleteArgs>(args: SelectSubset<T, ImagesDeleteArgs<ExtArgs>>): Prisma__ImagesClient<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "delete", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Update one Images.
     * @param {ImagesUpdateArgs} args - Arguments to update one Images.
     * @example
     * // Update one Images
     * const images = await prisma.images.update({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    update<T extends ImagesUpdateArgs>(args: SelectSubset<T, ImagesUpdateArgs<ExtArgs>>): Prisma__ImagesClient<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "update", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Delete zero or more Images.
     * @param {ImagesDeleteManyArgs} args - Arguments to filter Images to delete.
     * @example
     * // Delete a few Images
     * const { count } = await prisma.images.deleteMany({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     * 
     */
    deleteMany<T extends ImagesDeleteManyArgs>(args?: SelectSubset<T, ImagesDeleteManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more Images.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {ImagesUpdateManyArgs} args - Arguments to update one or more rows.
     * @example
     * // Update many Images
     * const images = await prisma.images.updateMany({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    updateMany<T extends ImagesUpdateManyArgs>(args: SelectSubset<T, ImagesUpdateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more Images and returns the data updated in the database.
     * @param {ImagesUpdateManyAndReturnArgs} args - Arguments to update many Images.
     * @example
     * // Update many Images
     * const images = await prisma.images.updateManyAndReturn({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Update zero or more Images and only return the `id`
     * const imagesWithIdOnly = await prisma.images.updateManyAndReturn({
     *   select: { id: true },
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    updateManyAndReturn<T extends ImagesUpdateManyAndReturnArgs>(args: SelectSubset<T, ImagesUpdateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "updateManyAndReturn", GlobalOmitOptions>>

    /**
     * Create or update one Images.
     * @param {ImagesUpsertArgs} args - Arguments to update or create a Images.
     * @example
     * // Update or create a Images
     * const images = await prisma.images.upsert({
     *   create: {
     *     // ... data to create a Images
     *   },
     *   update: {
     *     // ... in case it already exists, update
     *   },
     *   where: {
     *     // ... the filter for the Images we want to update
     *   }
     * })
     */
    upsert<T extends ImagesUpsertArgs>(args: SelectSubset<T, ImagesUpsertArgs<ExtArgs>>): Prisma__ImagesClient<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "upsert", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>


    /**
     * Count the number of Images.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {ImagesCountArgs} args - Arguments to filter Images to count.
     * @example
     * // Count the number of Images
     * const count = await prisma.images.count({
     *   where: {
     *     // ... the filter for the Images we want to count
     *   }
     * })
    **/
    count<T extends ImagesCountArgs>(
      args?: Subset<T, ImagesCountArgs>,
    ): Prisma.PrismaPromise<
      T extends $Utils.Record<'select', any>
        ? T['select'] extends true
          ? number
          : GetScalarType<T['select'], ImagesCountAggregateOutputType>
        : number
    >

    /**
     * Allows you to perform aggregations operations on a Images.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {ImagesAggregateArgs} args - Select which aggregations you would like to apply and on what fields.
     * @example
     * // Ordered by age ascending
     * // Where email contains prisma.io
     * // Limited to the 10 users
     * const aggregations = await prisma.user.aggregate({
     *   _avg: {
     *     age: true,
     *   },
     *   where: {
     *     email: {
     *       contains: "prisma.io",
     *     },
     *   },
     *   orderBy: {
     *     age: "asc",
     *   },
     *   take: 10,
     * })
    **/
    aggregate<T extends ImagesAggregateArgs>(args: Subset<T, ImagesAggregateArgs>): Prisma.PrismaPromise<GetImagesAggregateType<T>>

    /**
     * Group by Images.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {ImagesGroupByArgs} args - Group by arguments.
     * @example
     * // Group by city, order by createdAt, get count
     * const result = await prisma.user.groupBy({
     *   by: ['city', 'createdAt'],
     *   orderBy: {
     *     createdAt: true
     *   },
     *   _count: {
     *     _all: true
     *   },
     * })
     * 
    **/
    groupBy<
      T extends ImagesGroupByArgs,
      HasSelectOrTake extends Or<
        Extends<'skip', Keys<T>>,
        Extends<'take', Keys<T>>
      >,
      OrderByArg extends True extends HasSelectOrTake
        ? { orderBy: ImagesGroupByArgs['orderBy'] }
        : { orderBy?: ImagesGroupByArgs['orderBy'] },
      OrderFields extends ExcludeUnderscoreKeys<Keys<MaybeTupleToUnion<T['orderBy']>>>,
      ByFields extends MaybeTupleToUnion<T['by']>,
      ByValid extends Has<ByFields, OrderFields>,
      HavingFields extends GetHavingFields<T['having']>,
      HavingValid extends Has<ByFields, HavingFields>,
      ByEmpty extends T['by'] extends never[] ? True : False,
      InputErrors extends ByEmpty extends True
      ? `Error: "by" must not be empty.`
      : HavingValid extends False
      ? {
          [P in HavingFields]: P extends ByFields
            ? never
            : P extends string
            ? `Error: Field "${P}" used in "having" needs to be provided in "by".`
            : [
                Error,
                'Field ',
                P,
                ` in "having" needs to be provided in "by"`,
              ]
        }[HavingFields]
      : 'take' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "take", you also need to provide "orderBy"'
      : 'skip' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "skip", you also need to provide "orderBy"'
      : ByValid extends True
      ? {}
      : {
          [P in OrderFields]: P extends ByFields
            ? never
            : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
        }[OrderFields]
    >(args: SubsetIntersection<T, ImagesGroupByArgs, OrderByArg> & InputErrors): {} extends InputErrors ? GetImagesGroupByPayload<T> : Prisma.PrismaPromise<InputErrors>
  /**
   * Fields of the Images model
   */
  readonly fields: ImagesFieldRefs;
  }

  /**
   * The delegate class that acts as a "Promise-like" for Images.
   * Why is this prefixed with `Prisma__`?
   * Because we want to prevent naming conflicts as mentioned in
   * https://github.com/prisma/prisma-client-js/issues/707
   */
  export interface Prisma__ImagesClient<T, Null = never, ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> extends Prisma.PrismaPromise<T> {
    readonly [Symbol.toStringTag]: "PrismaPromise"
    user<T extends UsersDefaultArgs<ExtArgs> = {}>(args?: Subset<T, UsersDefaultArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions> | Null, Null, ExtArgs, GlobalOmitOptions>
    training_model<T extends TrainingModelsDefaultArgs<ExtArgs> = {}>(args?: Subset<T, TrainingModelsDefaultArgs<ExtArgs>>): Prisma__TrainingModelsClient<$Result.GetResult<Prisma.$TrainingModelsPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions> | Null, Null, ExtArgs, GlobalOmitOptions>
    pack<T extends Images$packArgs<ExtArgs> = {}>(args?: Subset<T, Images$packArgs<ExtArgs>>): Prisma__PacksClient<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>
    /**
     * Attaches callbacks for the resolution and/or rejection of the Promise.
     * @param onfulfilled The callback to execute when the Promise is resolved.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of which ever callback is executed.
     */
    then<TResult1 = T, TResult2 = never>(onfulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): $Utils.JsPromise<TResult1 | TResult2>
    /**
     * Attaches a callback for only the rejection of the Promise.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of the callback.
     */
    catch<TResult = never>(onrejected?: ((reason: any) => TResult | PromiseLike<TResult>) | undefined | null): $Utils.JsPromise<T | TResult>
    /**
     * Attaches a callback that is invoked when the Promise is settled (fulfilled or rejected). The
     * resolved value cannot be modified from the callback.
     * @param onfinally The callback to execute when the Promise is settled (fulfilled or rejected).
     * @returns A Promise for the completion of the callback.
     */
    finally(onfinally?: (() => void) | undefined | null): $Utils.JsPromise<T>
  }




  /**
   * Fields of the Images model
   */ 
  interface ImagesFieldRefs {
    readonly id: FieldRef<"Images", 'Int'>
    readonly pid: FieldRef<"Images", 'String'>
    readonly user_id: FieldRef<"Images", 'Int'>
    readonly training_model_id: FieldRef<"Images", 'Int'>
    readonly pack_id: FieldRef<"Images", 'Int'>
    readonly user_prompt: FieldRef<"Images", 'String'>
    readonly sys_prompt: FieldRef<"Images", 'String'>
    readonly alt: FieldRef<"Images", 'String'>
    readonly num_inference_steps: FieldRef<"Images", 'Int'>
    readonly content_type: FieldRef<"Images", 'ImageFormat'>
    readonly status: FieldRef<"Images", 'Status'>
    readonly image_size: FieldRef<"Images", 'ImageSize'>
    readonly fal_ai_request_id: FieldRef<"Images", 'String'>
    readonly width: FieldRef<"Images", 'Int'>
    readonly height: FieldRef<"Images", 'Int'>
    readonly image_url: FieldRef<"Images", 'String'>
    readonly image_url_s3: FieldRef<"Images", 'String'>
    readonly is_favorite: FieldRef<"Images", 'Boolean'>
    readonly deleted_at: FieldRef<"Images", 'DateTime'>
    readonly created_at: FieldRef<"Images", 'DateTime'>
    readonly updated_at: FieldRef<"Images", 'DateTime'>
  }
    

  // Custom InputTypes
  /**
   * Images findUnique
   */
  export type ImagesFindUniqueArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    /**
     * Filter, which Images to fetch.
     */
    where: ImagesWhereUniqueInput
  }

  /**
   * Images findUniqueOrThrow
   */
  export type ImagesFindUniqueOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    /**
     * Filter, which Images to fetch.
     */
    where: ImagesWhereUniqueInput
  }

  /**
   * Images findFirst
   */
  export type ImagesFindFirstArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    /**
     * Filter, which Images to fetch.
     */
    where?: ImagesWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Images to fetch.
     */
    orderBy?: ImagesOrderByWithRelationInput | ImagesOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for Images.
     */
    cursor?: ImagesWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Images from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Images.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of Images.
     */
    distinct?: ImagesScalarFieldEnum | ImagesScalarFieldEnum[]
  }

  /**
   * Images findFirstOrThrow
   */
  export type ImagesFindFirstOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    /**
     * Filter, which Images to fetch.
     */
    where?: ImagesWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Images to fetch.
     */
    orderBy?: ImagesOrderByWithRelationInput | ImagesOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for Images.
     */
    cursor?: ImagesWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Images from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Images.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of Images.
     */
    distinct?: ImagesScalarFieldEnum | ImagesScalarFieldEnum[]
  }

  /**
   * Images findMany
   */
  export type ImagesFindManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    /**
     * Filter, which Images to fetch.
     */
    where?: ImagesWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Images to fetch.
     */
    orderBy?: ImagesOrderByWithRelationInput | ImagesOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for listing Images.
     */
    cursor?: ImagesWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Images from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Images.
     */
    skip?: number
    distinct?: ImagesScalarFieldEnum | ImagesScalarFieldEnum[]
  }

  /**
   * Images create
   */
  export type ImagesCreateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    /**
     * The data needed to create a Images.
     */
    data: XOR<ImagesCreateInput, ImagesUncheckedCreateInput>
  }

  /**
   * Images createMany
   */
  export type ImagesCreateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to create many Images.
     */
    data: ImagesCreateManyInput | ImagesCreateManyInput[]
    skipDuplicates?: boolean
  }

  /**
   * Images createManyAndReturn
   */
  export type ImagesCreateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelectCreateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * The data used to create many Images.
     */
    data: ImagesCreateManyInput | ImagesCreateManyInput[]
    skipDuplicates?: boolean
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesIncludeCreateManyAndReturn<ExtArgs> | null
  }

  /**
   * Images update
   */
  export type ImagesUpdateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    /**
     * The data needed to update a Images.
     */
    data: XOR<ImagesUpdateInput, ImagesUncheckedUpdateInput>
    /**
     * Choose, which Images to update.
     */
    where: ImagesWhereUniqueInput
  }

  /**
   * Images updateMany
   */
  export type ImagesUpdateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to update Images.
     */
    data: XOR<ImagesUpdateManyMutationInput, ImagesUncheckedUpdateManyInput>
    /**
     * Filter which Images to update
     */
    where?: ImagesWhereInput
    /**
     * Limit how many Images to update.
     */
    limit?: number
  }

  /**
   * Images updateManyAndReturn
   */
  export type ImagesUpdateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelectUpdateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * The data used to update Images.
     */
    data: XOR<ImagesUpdateManyMutationInput, ImagesUncheckedUpdateManyInput>
    /**
     * Filter which Images to update
     */
    where?: ImagesWhereInput
    /**
     * Limit how many Images to update.
     */
    limit?: number
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesIncludeUpdateManyAndReturn<ExtArgs> | null
  }

  /**
   * Images upsert
   */
  export type ImagesUpsertArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    /**
     * The filter to search for the Images to update in case it exists.
     */
    where: ImagesWhereUniqueInput
    /**
     * In case the Images found by the `where` argument doesn't exist, create a new Images with this data.
     */
    create: XOR<ImagesCreateInput, ImagesUncheckedCreateInput>
    /**
     * In case the Images was found with the provided `where` argument, update it with this data.
     */
    update: XOR<ImagesUpdateInput, ImagesUncheckedUpdateInput>
  }

  /**
   * Images delete
   */
  export type ImagesDeleteArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    /**
     * Filter which Images to delete.
     */
    where: ImagesWhereUniqueInput
  }

  /**
   * Images deleteMany
   */
  export type ImagesDeleteManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which Images to delete
     */
    where?: ImagesWhereInput
    /**
     * Limit how many Images to delete.
     */
    limit?: number
  }

  /**
   * Images.pack
   */
  export type Images$packArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
    where?: PacksWhereInput
  }

  /**
   * Images without action
   */
  export type ImagesDefaultArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
  }


  /**
   * Model Transactions
   */

  export type AggregateTransactions = {
    _count: TransactionsCountAggregateOutputType | null
    _avg: TransactionsAvgAggregateOutputType | null
    _sum: TransactionsSumAggregateOutputType | null
    _min: TransactionsMinAggregateOutputType | null
    _max: TransactionsMaxAggregateOutputType | null
  }

  export type TransactionsAvgAggregateOutputType = {
    id: number | null
    user_id: number | null
    credit_amount: number | null
    model_amount: number | null
  }

  export type TransactionsSumAggregateOutputType = {
    id: number | null
    user_id: number | null
    credit_amount: number | null
    model_amount: number | null
  }

  export type TransactionsMinAggregateOutputType = {
    id: number | null
    pid: string | null
    user_id: number | null
    credit_amount: number | null
    model_amount: number | null
    currency: string | null
    payment_id: string | null
    order_id: string | null
    plan: string | null
    status: $Enums.Status | null
    created_at: Date | null
    updated_at: Date | null
  }

  export type TransactionsMaxAggregateOutputType = {
    id: number | null
    pid: string | null
    user_id: number | null
    credit_amount: number | null
    model_amount: number | null
    currency: string | null
    payment_id: string | null
    order_id: string | null
    plan: string | null
    status: $Enums.Status | null
    created_at: Date | null
    updated_at: Date | null
  }

  export type TransactionsCountAggregateOutputType = {
    id: number
    pid: number
    user_id: number
    credit_amount: number
    model_amount: number
    currency: number
    payment_id: number
    order_id: number
    plan: number
    status: number
    created_at: number
    updated_at: number
    _all: number
  }


  export type TransactionsAvgAggregateInputType = {
    id?: true
    user_id?: true
    credit_amount?: true
    model_amount?: true
  }

  export type TransactionsSumAggregateInputType = {
    id?: true
    user_id?: true
    credit_amount?: true
    model_amount?: true
  }

  export type TransactionsMinAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    credit_amount?: true
    model_amount?: true
    currency?: true
    payment_id?: true
    order_id?: true
    plan?: true
    status?: true
    created_at?: true
    updated_at?: true
  }

  export type TransactionsMaxAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    credit_amount?: true
    model_amount?: true
    currency?: true
    payment_id?: true
    order_id?: true
    plan?: true
    status?: true
    created_at?: true
    updated_at?: true
  }

  export type TransactionsCountAggregateInputType = {
    id?: true
    pid?: true
    user_id?: true
    credit_amount?: true
    model_amount?: true
    currency?: true
    payment_id?: true
    order_id?: true
    plan?: true
    status?: true
    created_at?: true
    updated_at?: true
    _all?: true
  }

  export type TransactionsAggregateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which Transactions to aggregate.
     */
    where?: TransactionsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Transactions to fetch.
     */
    orderBy?: TransactionsOrderByWithRelationInput | TransactionsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the start position
     */
    cursor?: TransactionsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Transactions from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Transactions.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Count returned Transactions
    **/
    _count?: true | TransactionsCountAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to average
    **/
    _avg?: TransactionsAvgAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to sum
    **/
    _sum?: TransactionsSumAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the minimum value
    **/
    _min?: TransactionsMinAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the maximum value
    **/
    _max?: TransactionsMaxAggregateInputType
  }

  export type GetTransactionsAggregateType<T extends TransactionsAggregateArgs> = {
        [P in keyof T & keyof AggregateTransactions]: P extends '_count' | 'count'
      ? T[P] extends true
        ? number
        : GetScalarType<T[P], AggregateTransactions[P]>
      : GetScalarType<T[P], AggregateTransactions[P]>
  }




  export type TransactionsGroupByArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: TransactionsWhereInput
    orderBy?: TransactionsOrderByWithAggregationInput | TransactionsOrderByWithAggregationInput[]
    by: TransactionsScalarFieldEnum[] | TransactionsScalarFieldEnum
    having?: TransactionsScalarWhereWithAggregatesInput
    take?: number
    skip?: number
    _count?: TransactionsCountAggregateInputType | true
    _avg?: TransactionsAvgAggregateInputType
    _sum?: TransactionsSumAggregateInputType
    _min?: TransactionsMinAggregateInputType
    _max?: TransactionsMaxAggregateInputType
  }

  export type TransactionsGroupByOutputType = {
    id: number
    pid: string
    user_id: number
    credit_amount: number
    model_amount: number
    currency: string
    payment_id: string
    order_id: string
    plan: string
    status: $Enums.Status
    created_at: Date
    updated_at: Date
    _count: TransactionsCountAggregateOutputType | null
    _avg: TransactionsAvgAggregateOutputType | null
    _sum: TransactionsSumAggregateOutputType | null
    _min: TransactionsMinAggregateOutputType | null
    _max: TransactionsMaxAggregateOutputType | null
  }

  type GetTransactionsGroupByPayload<T extends TransactionsGroupByArgs> = Prisma.PrismaPromise<
    Array<
      PickEnumerable<TransactionsGroupByOutputType, T['by']> &
        {
          [P in ((keyof T) & (keyof TransactionsGroupByOutputType))]: P extends '_count'
            ? T[P] extends boolean
              ? number
              : GetScalarType<T[P], TransactionsGroupByOutputType[P]>
            : GetScalarType<T[P], TransactionsGroupByOutputType[P]>
        }
      >
    >


  export type TransactionsSelect<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    credit_amount?: boolean
    model_amount?: boolean
    currency?: boolean
    payment_id?: boolean
    order_id?: boolean
    plan?: boolean
    status?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["transactions"]>

  export type TransactionsSelectCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    credit_amount?: boolean
    model_amount?: boolean
    currency?: boolean
    payment_id?: boolean
    order_id?: boolean
    plan?: boolean
    status?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["transactions"]>

  export type TransactionsSelectUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    user_id?: boolean
    credit_amount?: boolean
    model_amount?: boolean
    currency?: boolean
    payment_id?: boolean
    order_id?: boolean
    plan?: boolean
    status?: boolean
    created_at?: boolean
    updated_at?: boolean
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["transactions"]>

  export type TransactionsSelectScalar = {
    id?: boolean
    pid?: boolean
    user_id?: boolean
    credit_amount?: boolean
    model_amount?: boolean
    currency?: boolean
    payment_id?: boolean
    order_id?: boolean
    plan?: boolean
    status?: boolean
    created_at?: boolean
    updated_at?: boolean
  }

  export type TransactionsOmit<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetOmit<"id" | "pid" | "user_id" | "credit_amount" | "model_amount" | "currency" | "payment_id" | "order_id" | "plan" | "status" | "created_at" | "updated_at", ExtArgs["result"]["transactions"]>
  export type TransactionsInclude<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }
  export type TransactionsIncludeCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }
  export type TransactionsIncludeUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    user?: boolean | UsersDefaultArgs<ExtArgs>
  }

  export type $TransactionsPayload<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    name: "Transactions"
    objects: {
      user: Prisma.$UsersPayload<ExtArgs>
    }
    scalars: $Extensions.GetPayloadResult<{
      id: number
      pid: string
      user_id: number
      credit_amount: number
      model_amount: number
      currency: string
      payment_id: string
      order_id: string
      plan: string
      status: $Enums.Status
      created_at: Date
      updated_at: Date
    }, ExtArgs["result"]["transactions"]>
    composites: {}
  }

  type TransactionsGetPayload<S extends boolean | null | undefined | TransactionsDefaultArgs> = $Result.GetResult<Prisma.$TransactionsPayload, S>

  type TransactionsCountArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> =
    Omit<TransactionsFindManyArgs, 'select' | 'include' | 'distinct' | 'omit'> & {
      select?: TransactionsCountAggregateInputType | true
    }

  export interface TransactionsDelegate<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> {
    [K: symbol]: { types: Prisma.TypeMap<ExtArgs>['model']['Transactions'], meta: { name: 'Transactions' } }
    /**
     * Find zero or one Transactions that matches the filter.
     * @param {TransactionsFindUniqueArgs} args - Arguments to find a Transactions
     * @example
     * // Get one Transactions
     * const transactions = await prisma.transactions.findUnique({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUnique<T extends TransactionsFindUniqueArgs>(args: SelectSubset<T, TransactionsFindUniqueArgs<ExtArgs>>): Prisma__TransactionsClient<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "findUnique", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find one Transactions that matches the filter or throw an error with `error.code='P2025'`
     * if no matches were found.
     * @param {TransactionsFindUniqueOrThrowArgs} args - Arguments to find a Transactions
     * @example
     * // Get one Transactions
     * const transactions = await prisma.transactions.findUniqueOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUniqueOrThrow<T extends TransactionsFindUniqueOrThrowArgs>(args: SelectSubset<T, TransactionsFindUniqueOrThrowArgs<ExtArgs>>): Prisma__TransactionsClient<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first Transactions that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TransactionsFindFirstArgs} args - Arguments to find a Transactions
     * @example
     * // Get one Transactions
     * const transactions = await prisma.transactions.findFirst({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirst<T extends TransactionsFindFirstArgs>(args?: SelectSubset<T, TransactionsFindFirstArgs<ExtArgs>>): Prisma__TransactionsClient<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "findFirst", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first Transactions that matches the filter or
     * throw `PrismaKnownClientError` with `P2025` code if no matches were found.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TransactionsFindFirstOrThrowArgs} args - Arguments to find a Transactions
     * @example
     * // Get one Transactions
     * const transactions = await prisma.transactions.findFirstOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirstOrThrow<T extends TransactionsFindFirstOrThrowArgs>(args?: SelectSubset<T, TransactionsFindFirstOrThrowArgs<ExtArgs>>): Prisma__TransactionsClient<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "findFirstOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find zero or more Transactions that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TransactionsFindManyArgs} args - Arguments to filter and select certain fields only.
     * @example
     * // Get all Transactions
     * const transactions = await prisma.transactions.findMany()
     * 
     * // Get first 10 Transactions
     * const transactions = await prisma.transactions.findMany({ take: 10 })
     * 
     * // Only select the `id`
     * const transactionsWithIdOnly = await prisma.transactions.findMany({ select: { id: true } })
     * 
     */
    findMany<T extends TransactionsFindManyArgs>(args?: SelectSubset<T, TransactionsFindManyArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "findMany", GlobalOmitOptions>>

    /**
     * Create a Transactions.
     * @param {TransactionsCreateArgs} args - Arguments to create a Transactions.
     * @example
     * // Create one Transactions
     * const Transactions = await prisma.transactions.create({
     *   data: {
     *     // ... data to create a Transactions
     *   }
     * })
     * 
     */
    create<T extends TransactionsCreateArgs>(args: SelectSubset<T, TransactionsCreateArgs<ExtArgs>>): Prisma__TransactionsClient<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "create", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Create many Transactions.
     * @param {TransactionsCreateManyArgs} args - Arguments to create many Transactions.
     * @example
     * // Create many Transactions
     * const transactions = await prisma.transactions.createMany({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     *     
     */
    createMany<T extends TransactionsCreateManyArgs>(args?: SelectSubset<T, TransactionsCreateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Create many Transactions and returns the data saved in the database.
     * @param {TransactionsCreateManyAndReturnArgs} args - Arguments to create many Transactions.
     * @example
     * // Create many Transactions
     * const transactions = await prisma.transactions.createManyAndReturn({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Create many Transactions and only return the `id`
     * const transactionsWithIdOnly = await prisma.transactions.createManyAndReturn({
     *   select: { id: true },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    createManyAndReturn<T extends TransactionsCreateManyAndReturnArgs>(args?: SelectSubset<T, TransactionsCreateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "createManyAndReturn", GlobalOmitOptions>>

    /**
     * Delete a Transactions.
     * @param {TransactionsDeleteArgs} args - Arguments to delete one Transactions.
     * @example
     * // Delete one Transactions
     * const Transactions = await prisma.transactions.delete({
     *   where: {
     *     // ... filter to delete one Transactions
     *   }
     * })
     * 
     */
    delete<T extends TransactionsDeleteArgs>(args: SelectSubset<T, TransactionsDeleteArgs<ExtArgs>>): Prisma__TransactionsClient<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "delete", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Update one Transactions.
     * @param {TransactionsUpdateArgs} args - Arguments to update one Transactions.
     * @example
     * // Update one Transactions
     * const transactions = await prisma.transactions.update({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    update<T extends TransactionsUpdateArgs>(args: SelectSubset<T, TransactionsUpdateArgs<ExtArgs>>): Prisma__TransactionsClient<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "update", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Delete zero or more Transactions.
     * @param {TransactionsDeleteManyArgs} args - Arguments to filter Transactions to delete.
     * @example
     * // Delete a few Transactions
     * const { count } = await prisma.transactions.deleteMany({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     * 
     */
    deleteMany<T extends TransactionsDeleteManyArgs>(args?: SelectSubset<T, TransactionsDeleteManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more Transactions.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TransactionsUpdateManyArgs} args - Arguments to update one or more rows.
     * @example
     * // Update many Transactions
     * const transactions = await prisma.transactions.updateMany({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    updateMany<T extends TransactionsUpdateManyArgs>(args: SelectSubset<T, TransactionsUpdateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more Transactions and returns the data updated in the database.
     * @param {TransactionsUpdateManyAndReturnArgs} args - Arguments to update many Transactions.
     * @example
     * // Update many Transactions
     * const transactions = await prisma.transactions.updateManyAndReturn({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Update zero or more Transactions and only return the `id`
     * const transactionsWithIdOnly = await prisma.transactions.updateManyAndReturn({
     *   select: { id: true },
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    updateManyAndReturn<T extends TransactionsUpdateManyAndReturnArgs>(args: SelectSubset<T, TransactionsUpdateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "updateManyAndReturn", GlobalOmitOptions>>

    /**
     * Create or update one Transactions.
     * @param {TransactionsUpsertArgs} args - Arguments to update or create a Transactions.
     * @example
     * // Update or create a Transactions
     * const transactions = await prisma.transactions.upsert({
     *   create: {
     *     // ... data to create a Transactions
     *   },
     *   update: {
     *     // ... in case it already exists, update
     *   },
     *   where: {
     *     // ... the filter for the Transactions we want to update
     *   }
     * })
     */
    upsert<T extends TransactionsUpsertArgs>(args: SelectSubset<T, TransactionsUpsertArgs<ExtArgs>>): Prisma__TransactionsClient<$Result.GetResult<Prisma.$TransactionsPayload<ExtArgs>, T, "upsert", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>


    /**
     * Count the number of Transactions.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TransactionsCountArgs} args - Arguments to filter Transactions to count.
     * @example
     * // Count the number of Transactions
     * const count = await prisma.transactions.count({
     *   where: {
     *     // ... the filter for the Transactions we want to count
     *   }
     * })
    **/
    count<T extends TransactionsCountArgs>(
      args?: Subset<T, TransactionsCountArgs>,
    ): Prisma.PrismaPromise<
      T extends $Utils.Record<'select', any>
        ? T['select'] extends true
          ? number
          : GetScalarType<T['select'], TransactionsCountAggregateOutputType>
        : number
    >

    /**
     * Allows you to perform aggregations operations on a Transactions.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TransactionsAggregateArgs} args - Select which aggregations you would like to apply and on what fields.
     * @example
     * // Ordered by age ascending
     * // Where email contains prisma.io
     * // Limited to the 10 users
     * const aggregations = await prisma.user.aggregate({
     *   _avg: {
     *     age: true,
     *   },
     *   where: {
     *     email: {
     *       contains: "prisma.io",
     *     },
     *   },
     *   orderBy: {
     *     age: "asc",
     *   },
     *   take: 10,
     * })
    **/
    aggregate<T extends TransactionsAggregateArgs>(args: Subset<T, TransactionsAggregateArgs>): Prisma.PrismaPromise<GetTransactionsAggregateType<T>>

    /**
     * Group by Transactions.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {TransactionsGroupByArgs} args - Group by arguments.
     * @example
     * // Group by city, order by createdAt, get count
     * const result = await prisma.user.groupBy({
     *   by: ['city', 'createdAt'],
     *   orderBy: {
     *     createdAt: true
     *   },
     *   _count: {
     *     _all: true
     *   },
     * })
     * 
    **/
    groupBy<
      T extends TransactionsGroupByArgs,
      HasSelectOrTake extends Or<
        Extends<'skip', Keys<T>>,
        Extends<'take', Keys<T>>
      >,
      OrderByArg extends True extends HasSelectOrTake
        ? { orderBy: TransactionsGroupByArgs['orderBy'] }
        : { orderBy?: TransactionsGroupByArgs['orderBy'] },
      OrderFields extends ExcludeUnderscoreKeys<Keys<MaybeTupleToUnion<T['orderBy']>>>,
      ByFields extends MaybeTupleToUnion<T['by']>,
      ByValid extends Has<ByFields, OrderFields>,
      HavingFields extends GetHavingFields<T['having']>,
      HavingValid extends Has<ByFields, HavingFields>,
      ByEmpty extends T['by'] extends never[] ? True : False,
      InputErrors extends ByEmpty extends True
      ? `Error: "by" must not be empty.`
      : HavingValid extends False
      ? {
          [P in HavingFields]: P extends ByFields
            ? never
            : P extends string
            ? `Error: Field "${P}" used in "having" needs to be provided in "by".`
            : [
                Error,
                'Field ',
                P,
                ` in "having" needs to be provided in "by"`,
              ]
        }[HavingFields]
      : 'take' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "take", you also need to provide "orderBy"'
      : 'skip' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "skip", you also need to provide "orderBy"'
      : ByValid extends True
      ? {}
      : {
          [P in OrderFields]: P extends ByFields
            ? never
            : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
        }[OrderFields]
    >(args: SubsetIntersection<T, TransactionsGroupByArgs, OrderByArg> & InputErrors): {} extends InputErrors ? GetTransactionsGroupByPayload<T> : Prisma.PrismaPromise<InputErrors>
  /**
   * Fields of the Transactions model
   */
  readonly fields: TransactionsFieldRefs;
  }

  /**
   * The delegate class that acts as a "Promise-like" for Transactions.
   * Why is this prefixed with `Prisma__`?
   * Because we want to prevent naming conflicts as mentioned in
   * https://github.com/prisma/prisma-client-js/issues/707
   */
  export interface Prisma__TransactionsClient<T, Null = never, ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> extends Prisma.PrismaPromise<T> {
    readonly [Symbol.toStringTag]: "PrismaPromise"
    user<T extends UsersDefaultArgs<ExtArgs> = {}>(args?: Subset<T, UsersDefaultArgs<ExtArgs>>): Prisma__UsersClient<$Result.GetResult<Prisma.$UsersPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions> | Null, Null, ExtArgs, GlobalOmitOptions>
    /**
     * Attaches callbacks for the resolution and/or rejection of the Promise.
     * @param onfulfilled The callback to execute when the Promise is resolved.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of which ever callback is executed.
     */
    then<TResult1 = T, TResult2 = never>(onfulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): $Utils.JsPromise<TResult1 | TResult2>
    /**
     * Attaches a callback for only the rejection of the Promise.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of the callback.
     */
    catch<TResult = never>(onrejected?: ((reason: any) => TResult | PromiseLike<TResult>) | undefined | null): $Utils.JsPromise<T | TResult>
    /**
     * Attaches a callback that is invoked when the Promise is settled (fulfilled or rejected). The
     * resolved value cannot be modified from the callback.
     * @param onfinally The callback to execute when the Promise is settled (fulfilled or rejected).
     * @returns A Promise for the completion of the callback.
     */
    finally(onfinally?: (() => void) | undefined | null): $Utils.JsPromise<T>
  }




  /**
   * Fields of the Transactions model
   */ 
  interface TransactionsFieldRefs {
    readonly id: FieldRef<"Transactions", 'Int'>
    readonly pid: FieldRef<"Transactions", 'String'>
    readonly user_id: FieldRef<"Transactions", 'Int'>
    readonly credit_amount: FieldRef<"Transactions", 'Int'>
    readonly model_amount: FieldRef<"Transactions", 'Int'>
    readonly currency: FieldRef<"Transactions", 'String'>
    readonly payment_id: FieldRef<"Transactions", 'String'>
    readonly order_id: FieldRef<"Transactions", 'String'>
    readonly plan: FieldRef<"Transactions", 'String'>
    readonly status: FieldRef<"Transactions", 'Status'>
    readonly created_at: FieldRef<"Transactions", 'DateTime'>
    readonly updated_at: FieldRef<"Transactions", 'DateTime'>
  }
    

  // Custom InputTypes
  /**
   * Transactions findUnique
   */
  export type TransactionsFindUniqueArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
    /**
     * Filter, which Transactions to fetch.
     */
    where: TransactionsWhereUniqueInput
  }

  /**
   * Transactions findUniqueOrThrow
   */
  export type TransactionsFindUniqueOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
    /**
     * Filter, which Transactions to fetch.
     */
    where: TransactionsWhereUniqueInput
  }

  /**
   * Transactions findFirst
   */
  export type TransactionsFindFirstArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
    /**
     * Filter, which Transactions to fetch.
     */
    where?: TransactionsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Transactions to fetch.
     */
    orderBy?: TransactionsOrderByWithRelationInput | TransactionsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for Transactions.
     */
    cursor?: TransactionsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Transactions from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Transactions.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of Transactions.
     */
    distinct?: TransactionsScalarFieldEnum | TransactionsScalarFieldEnum[]
  }

  /**
   * Transactions findFirstOrThrow
   */
  export type TransactionsFindFirstOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
    /**
     * Filter, which Transactions to fetch.
     */
    where?: TransactionsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Transactions to fetch.
     */
    orderBy?: TransactionsOrderByWithRelationInput | TransactionsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for Transactions.
     */
    cursor?: TransactionsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Transactions from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Transactions.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of Transactions.
     */
    distinct?: TransactionsScalarFieldEnum | TransactionsScalarFieldEnum[]
  }

  /**
   * Transactions findMany
   */
  export type TransactionsFindManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
    /**
     * Filter, which Transactions to fetch.
     */
    where?: TransactionsWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Transactions to fetch.
     */
    orderBy?: TransactionsOrderByWithRelationInput | TransactionsOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for listing Transactions.
     */
    cursor?: TransactionsWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Transactions from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Transactions.
     */
    skip?: number
    distinct?: TransactionsScalarFieldEnum | TransactionsScalarFieldEnum[]
  }

  /**
   * Transactions create
   */
  export type TransactionsCreateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
    /**
     * The data needed to create a Transactions.
     */
    data: XOR<TransactionsCreateInput, TransactionsUncheckedCreateInput>
  }

  /**
   * Transactions createMany
   */
  export type TransactionsCreateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to create many Transactions.
     */
    data: TransactionsCreateManyInput | TransactionsCreateManyInput[]
    skipDuplicates?: boolean
  }

  /**
   * Transactions createManyAndReturn
   */
  export type TransactionsCreateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelectCreateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * The data used to create many Transactions.
     */
    data: TransactionsCreateManyInput | TransactionsCreateManyInput[]
    skipDuplicates?: boolean
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsIncludeCreateManyAndReturn<ExtArgs> | null
  }

  /**
   * Transactions update
   */
  export type TransactionsUpdateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
    /**
     * The data needed to update a Transactions.
     */
    data: XOR<TransactionsUpdateInput, TransactionsUncheckedUpdateInput>
    /**
     * Choose, which Transactions to update.
     */
    where: TransactionsWhereUniqueInput
  }

  /**
   * Transactions updateMany
   */
  export type TransactionsUpdateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to update Transactions.
     */
    data: XOR<TransactionsUpdateManyMutationInput, TransactionsUncheckedUpdateManyInput>
    /**
     * Filter which Transactions to update
     */
    where?: TransactionsWhereInput
    /**
     * Limit how many Transactions to update.
     */
    limit?: number
  }

  /**
   * Transactions updateManyAndReturn
   */
  export type TransactionsUpdateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelectUpdateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * The data used to update Transactions.
     */
    data: XOR<TransactionsUpdateManyMutationInput, TransactionsUncheckedUpdateManyInput>
    /**
     * Filter which Transactions to update
     */
    where?: TransactionsWhereInput
    /**
     * Limit how many Transactions to update.
     */
    limit?: number
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsIncludeUpdateManyAndReturn<ExtArgs> | null
  }

  /**
   * Transactions upsert
   */
  export type TransactionsUpsertArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
    /**
     * The filter to search for the Transactions to update in case it exists.
     */
    where: TransactionsWhereUniqueInput
    /**
     * In case the Transactions found by the `where` argument doesn't exist, create a new Transactions with this data.
     */
    create: XOR<TransactionsCreateInput, TransactionsUncheckedCreateInput>
    /**
     * In case the Transactions was found with the provided `where` argument, update it with this data.
     */
    update: XOR<TransactionsUpdateInput, TransactionsUncheckedUpdateInput>
  }

  /**
   * Transactions delete
   */
  export type TransactionsDeleteArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
    /**
     * Filter which Transactions to delete.
     */
    where: TransactionsWhereUniqueInput
  }

  /**
   * Transactions deleteMany
   */
  export type TransactionsDeleteManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which Transactions to delete
     */
    where?: TransactionsWhereInput
    /**
     * Limit how many Transactions to delete.
     */
    limit?: number
  }

  /**
   * Transactions without action
   */
  export type TransactionsDefaultArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Transactions
     */
    select?: TransactionsSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Transactions
     */
    omit?: TransactionsOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: TransactionsInclude<ExtArgs> | null
  }


  /**
   * Model Packs
   */

  export type AggregatePacks = {
    _count: PacksCountAggregateOutputType | null
    _avg: PacksAvgAggregateOutputType | null
    _sum: PacksSumAggregateOutputType | null
    _min: PacksMinAggregateOutputType | null
    _max: PacksMaxAggregateOutputType | null
  }

  export type PacksAvgAggregateOutputType = {
    id: number | null
    credits: number | null
    amount: number | null
  }

  export type PacksSumAggregateOutputType = {
    id: number | null
    credits: number | null
    amount: number | null
  }

  export type PacksMinAggregateOutputType = {
    id: number | null
    pid: string | null
    name: string | null
    description: string | null
    pack_prompts: string | null
    credits: number | null
    amount: number | null
    image_url: string | null
    created_at: Date | null
    updated_at: Date | null
  }

  export type PacksMaxAggregateOutputType = {
    id: number | null
    pid: string | null
    name: string | null
    description: string | null
    pack_prompts: string | null
    credits: number | null
    amount: number | null
    image_url: string | null
    created_at: Date | null
    updated_at: Date | null
  }

  export type PacksCountAggregateOutputType = {
    id: number
    pid: number
    name: number
    description: number
    pack_prompts: number
    credits: number
    amount: number
    image_url: number
    created_at: number
    updated_at: number
    _all: number
  }


  export type PacksAvgAggregateInputType = {
    id?: true
    credits?: true
    amount?: true
  }

  export type PacksSumAggregateInputType = {
    id?: true
    credits?: true
    amount?: true
  }

  export type PacksMinAggregateInputType = {
    id?: true
    pid?: true
    name?: true
    description?: true
    pack_prompts?: true
    credits?: true
    amount?: true
    image_url?: true
    created_at?: true
    updated_at?: true
  }

  export type PacksMaxAggregateInputType = {
    id?: true
    pid?: true
    name?: true
    description?: true
    pack_prompts?: true
    credits?: true
    amount?: true
    image_url?: true
    created_at?: true
    updated_at?: true
  }

  export type PacksCountAggregateInputType = {
    id?: true
    pid?: true
    name?: true
    description?: true
    pack_prompts?: true
    credits?: true
    amount?: true
    image_url?: true
    created_at?: true
    updated_at?: true
    _all?: true
  }

  export type PacksAggregateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which Packs to aggregate.
     */
    where?: PacksWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Packs to fetch.
     */
    orderBy?: PacksOrderByWithRelationInput | PacksOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the start position
     */
    cursor?: PacksWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Packs from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Packs.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Count returned Packs
    **/
    _count?: true | PacksCountAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to average
    **/
    _avg?: PacksAvgAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to sum
    **/
    _sum?: PacksSumAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the minimum value
    **/
    _min?: PacksMinAggregateInputType
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/aggregations Aggregation Docs}
     * 
     * Select which fields to find the maximum value
    **/
    _max?: PacksMaxAggregateInputType
  }

  export type GetPacksAggregateType<T extends PacksAggregateArgs> = {
        [P in keyof T & keyof AggregatePacks]: P extends '_count' | 'count'
      ? T[P] extends true
        ? number
        : GetScalarType<T[P], AggregatePacks[P]>
      : GetScalarType<T[P], AggregatePacks[P]>
  }




  export type PacksGroupByArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    where?: PacksWhereInput
    orderBy?: PacksOrderByWithAggregationInput | PacksOrderByWithAggregationInput[]
    by: PacksScalarFieldEnum[] | PacksScalarFieldEnum
    having?: PacksScalarWhereWithAggregatesInput
    take?: number
    skip?: number
    _count?: PacksCountAggregateInputType | true
    _avg?: PacksAvgAggregateInputType
    _sum?: PacksSumAggregateInputType
    _min?: PacksMinAggregateInputType
    _max?: PacksMaxAggregateInputType
  }

  export type PacksGroupByOutputType = {
    id: number
    pid: string
    name: string
    description: string
    pack_prompts: string
    credits: number
    amount: number
    image_url: string
    created_at: Date
    updated_at: Date
    _count: PacksCountAggregateOutputType | null
    _avg: PacksAvgAggregateOutputType | null
    _sum: PacksSumAggregateOutputType | null
    _min: PacksMinAggregateOutputType | null
    _max: PacksMaxAggregateOutputType | null
  }

  type GetPacksGroupByPayload<T extends PacksGroupByArgs> = Prisma.PrismaPromise<
    Array<
      PickEnumerable<PacksGroupByOutputType, T['by']> &
        {
          [P in ((keyof T) & (keyof PacksGroupByOutputType))]: P extends '_count'
            ? T[P] extends boolean
              ? number
              : GetScalarType<T[P], PacksGroupByOutputType[P]>
            : GetScalarType<T[P], PacksGroupByOutputType[P]>
        }
      >
    >


  export type PacksSelect<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    name?: boolean
    description?: boolean
    pack_prompts?: boolean
    credits?: boolean
    amount?: boolean
    image_url?: boolean
    created_at?: boolean
    updated_at?: boolean
    Images?: boolean | Packs$ImagesArgs<ExtArgs>
    _count?: boolean | PacksCountOutputTypeDefaultArgs<ExtArgs>
  }, ExtArgs["result"]["packs"]>

  export type PacksSelectCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    name?: boolean
    description?: boolean
    pack_prompts?: boolean
    credits?: boolean
    amount?: boolean
    image_url?: boolean
    created_at?: boolean
    updated_at?: boolean
  }, ExtArgs["result"]["packs"]>

  export type PacksSelectUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetSelect<{
    id?: boolean
    pid?: boolean
    name?: boolean
    description?: boolean
    pack_prompts?: boolean
    credits?: boolean
    amount?: boolean
    image_url?: boolean
    created_at?: boolean
    updated_at?: boolean
  }, ExtArgs["result"]["packs"]>

  export type PacksSelectScalar = {
    id?: boolean
    pid?: boolean
    name?: boolean
    description?: boolean
    pack_prompts?: boolean
    credits?: boolean
    amount?: boolean
    image_url?: boolean
    created_at?: boolean
    updated_at?: boolean
  }

  export type PacksOmit<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = $Extensions.GetOmit<"id" | "pid" | "name" | "description" | "pack_prompts" | "credits" | "amount" | "image_url" | "created_at" | "updated_at", ExtArgs["result"]["packs"]>
  export type PacksInclude<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    Images?: boolean | Packs$ImagesArgs<ExtArgs>
    _count?: boolean | PacksCountOutputTypeDefaultArgs<ExtArgs>
  }
  export type PacksIncludeCreateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {}
  export type PacksIncludeUpdateManyAndReturn<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {}

  export type $PacksPayload<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    name: "Packs"
    objects: {
      Images: Prisma.$ImagesPayload<ExtArgs>[]
    }
    scalars: $Extensions.GetPayloadResult<{
      id: number
      pid: string
      name: string
      description: string
      pack_prompts: string
      credits: number
      amount: number
      image_url: string
      created_at: Date
      updated_at: Date
    }, ExtArgs["result"]["packs"]>
    composites: {}
  }

  type PacksGetPayload<S extends boolean | null | undefined | PacksDefaultArgs> = $Result.GetResult<Prisma.$PacksPayload, S>

  type PacksCountArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> =
    Omit<PacksFindManyArgs, 'select' | 'include' | 'distinct' | 'omit'> & {
      select?: PacksCountAggregateInputType | true
    }

  export interface PacksDelegate<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> {
    [K: symbol]: { types: Prisma.TypeMap<ExtArgs>['model']['Packs'], meta: { name: 'Packs' } }
    /**
     * Find zero or one Packs that matches the filter.
     * @param {PacksFindUniqueArgs} args - Arguments to find a Packs
     * @example
     * // Get one Packs
     * const packs = await prisma.packs.findUnique({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUnique<T extends PacksFindUniqueArgs>(args: SelectSubset<T, PacksFindUniqueArgs<ExtArgs>>): Prisma__PacksClient<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "findUnique", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find one Packs that matches the filter or throw an error with `error.code='P2025'`
     * if no matches were found.
     * @param {PacksFindUniqueOrThrowArgs} args - Arguments to find a Packs
     * @example
     * // Get one Packs
     * const packs = await prisma.packs.findUniqueOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findUniqueOrThrow<T extends PacksFindUniqueOrThrowArgs>(args: SelectSubset<T, PacksFindUniqueOrThrowArgs<ExtArgs>>): Prisma__PacksClient<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "findUniqueOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first Packs that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {PacksFindFirstArgs} args - Arguments to find a Packs
     * @example
     * // Get one Packs
     * const packs = await prisma.packs.findFirst({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirst<T extends PacksFindFirstArgs>(args?: SelectSubset<T, PacksFindFirstArgs<ExtArgs>>): Prisma__PacksClient<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "findFirst", GlobalOmitOptions> | null, null, ExtArgs, GlobalOmitOptions>

    /**
     * Find the first Packs that matches the filter or
     * throw `PrismaKnownClientError` with `P2025` code if no matches were found.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {PacksFindFirstOrThrowArgs} args - Arguments to find a Packs
     * @example
     * // Get one Packs
     * const packs = await prisma.packs.findFirstOrThrow({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     */
    findFirstOrThrow<T extends PacksFindFirstOrThrowArgs>(args?: SelectSubset<T, PacksFindFirstOrThrowArgs<ExtArgs>>): Prisma__PacksClient<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "findFirstOrThrow", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Find zero or more Packs that matches the filter.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {PacksFindManyArgs} args - Arguments to filter and select certain fields only.
     * @example
     * // Get all Packs
     * const packs = await prisma.packs.findMany()
     * 
     * // Get first 10 Packs
     * const packs = await prisma.packs.findMany({ take: 10 })
     * 
     * // Only select the `id`
     * const packsWithIdOnly = await prisma.packs.findMany({ select: { id: true } })
     * 
     */
    findMany<T extends PacksFindManyArgs>(args?: SelectSubset<T, PacksFindManyArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "findMany", GlobalOmitOptions>>

    /**
     * Create a Packs.
     * @param {PacksCreateArgs} args - Arguments to create a Packs.
     * @example
     * // Create one Packs
     * const Packs = await prisma.packs.create({
     *   data: {
     *     // ... data to create a Packs
     *   }
     * })
     * 
     */
    create<T extends PacksCreateArgs>(args: SelectSubset<T, PacksCreateArgs<ExtArgs>>): Prisma__PacksClient<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "create", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Create many Packs.
     * @param {PacksCreateManyArgs} args - Arguments to create many Packs.
     * @example
     * // Create many Packs
     * const packs = await prisma.packs.createMany({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     *     
     */
    createMany<T extends PacksCreateManyArgs>(args?: SelectSubset<T, PacksCreateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Create many Packs and returns the data saved in the database.
     * @param {PacksCreateManyAndReturnArgs} args - Arguments to create many Packs.
     * @example
     * // Create many Packs
     * const packs = await prisma.packs.createManyAndReturn({
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Create many Packs and only return the `id`
     * const packsWithIdOnly = await prisma.packs.createManyAndReturn({
     *   select: { id: true },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    createManyAndReturn<T extends PacksCreateManyAndReturnArgs>(args?: SelectSubset<T, PacksCreateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "createManyAndReturn", GlobalOmitOptions>>

    /**
     * Delete a Packs.
     * @param {PacksDeleteArgs} args - Arguments to delete one Packs.
     * @example
     * // Delete one Packs
     * const Packs = await prisma.packs.delete({
     *   where: {
     *     // ... filter to delete one Packs
     *   }
     * })
     * 
     */
    delete<T extends PacksDeleteArgs>(args: SelectSubset<T, PacksDeleteArgs<ExtArgs>>): Prisma__PacksClient<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "delete", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Update one Packs.
     * @param {PacksUpdateArgs} args - Arguments to update one Packs.
     * @example
     * // Update one Packs
     * const packs = await prisma.packs.update({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    update<T extends PacksUpdateArgs>(args: SelectSubset<T, PacksUpdateArgs<ExtArgs>>): Prisma__PacksClient<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "update", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>

    /**
     * Delete zero or more Packs.
     * @param {PacksDeleteManyArgs} args - Arguments to filter Packs to delete.
     * @example
     * // Delete a few Packs
     * const { count } = await prisma.packs.deleteMany({
     *   where: {
     *     // ... provide filter here
     *   }
     * })
     * 
     */
    deleteMany<T extends PacksDeleteManyArgs>(args?: SelectSubset<T, PacksDeleteManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more Packs.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {PacksUpdateManyArgs} args - Arguments to update one or more rows.
     * @example
     * // Update many Packs
     * const packs = await prisma.packs.updateMany({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: {
     *     // ... provide data here
     *   }
     * })
     * 
     */
    updateMany<T extends PacksUpdateManyArgs>(args: SelectSubset<T, PacksUpdateManyArgs<ExtArgs>>): Prisma.PrismaPromise<BatchPayload>

    /**
     * Update zero or more Packs and returns the data updated in the database.
     * @param {PacksUpdateManyAndReturnArgs} args - Arguments to update many Packs.
     * @example
     * // Update many Packs
     * const packs = await prisma.packs.updateManyAndReturn({
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * 
     * // Update zero or more Packs and only return the `id`
     * const packsWithIdOnly = await prisma.packs.updateManyAndReturn({
     *   select: { id: true },
     *   where: {
     *     // ... provide filter here
     *   },
     *   data: [
     *     // ... provide data here
     *   ]
     * })
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * 
     */
    updateManyAndReturn<T extends PacksUpdateManyAndReturnArgs>(args: SelectSubset<T, PacksUpdateManyAndReturnArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "updateManyAndReturn", GlobalOmitOptions>>

    /**
     * Create or update one Packs.
     * @param {PacksUpsertArgs} args - Arguments to update or create a Packs.
     * @example
     * // Update or create a Packs
     * const packs = await prisma.packs.upsert({
     *   create: {
     *     // ... data to create a Packs
     *   },
     *   update: {
     *     // ... in case it already exists, update
     *   },
     *   where: {
     *     // ... the filter for the Packs we want to update
     *   }
     * })
     */
    upsert<T extends PacksUpsertArgs>(args: SelectSubset<T, PacksUpsertArgs<ExtArgs>>): Prisma__PacksClient<$Result.GetResult<Prisma.$PacksPayload<ExtArgs>, T, "upsert", GlobalOmitOptions>, never, ExtArgs, GlobalOmitOptions>


    /**
     * Count the number of Packs.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {PacksCountArgs} args - Arguments to filter Packs to count.
     * @example
     * // Count the number of Packs
     * const count = await prisma.packs.count({
     *   where: {
     *     // ... the filter for the Packs we want to count
     *   }
     * })
    **/
    count<T extends PacksCountArgs>(
      args?: Subset<T, PacksCountArgs>,
    ): Prisma.PrismaPromise<
      T extends $Utils.Record<'select', any>
        ? T['select'] extends true
          ? number
          : GetScalarType<T['select'], PacksCountAggregateOutputType>
        : number
    >

    /**
     * Allows you to perform aggregations operations on a Packs.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {PacksAggregateArgs} args - Select which aggregations you would like to apply and on what fields.
     * @example
     * // Ordered by age ascending
     * // Where email contains prisma.io
     * // Limited to the 10 users
     * const aggregations = await prisma.user.aggregate({
     *   _avg: {
     *     age: true,
     *   },
     *   where: {
     *     email: {
     *       contains: "prisma.io",
     *     },
     *   },
     *   orderBy: {
     *     age: "asc",
     *   },
     *   take: 10,
     * })
    **/
    aggregate<T extends PacksAggregateArgs>(args: Subset<T, PacksAggregateArgs>): Prisma.PrismaPromise<GetPacksAggregateType<T>>

    /**
     * Group by Packs.
     * Note, that providing `undefined` is treated as the value not being there.
     * Read more here: https://pris.ly/d/null-undefined
     * @param {PacksGroupByArgs} args - Group by arguments.
     * @example
     * // Group by city, order by createdAt, get count
     * const result = await prisma.user.groupBy({
     *   by: ['city', 'createdAt'],
     *   orderBy: {
     *     createdAt: true
     *   },
     *   _count: {
     *     _all: true
     *   },
     * })
     * 
    **/
    groupBy<
      T extends PacksGroupByArgs,
      HasSelectOrTake extends Or<
        Extends<'skip', Keys<T>>,
        Extends<'take', Keys<T>>
      >,
      OrderByArg extends True extends HasSelectOrTake
        ? { orderBy: PacksGroupByArgs['orderBy'] }
        : { orderBy?: PacksGroupByArgs['orderBy'] },
      OrderFields extends ExcludeUnderscoreKeys<Keys<MaybeTupleToUnion<T['orderBy']>>>,
      ByFields extends MaybeTupleToUnion<T['by']>,
      ByValid extends Has<ByFields, OrderFields>,
      HavingFields extends GetHavingFields<T['having']>,
      HavingValid extends Has<ByFields, HavingFields>,
      ByEmpty extends T['by'] extends never[] ? True : False,
      InputErrors extends ByEmpty extends True
      ? `Error: "by" must not be empty.`
      : HavingValid extends False
      ? {
          [P in HavingFields]: P extends ByFields
            ? never
            : P extends string
            ? `Error: Field "${P}" used in "having" needs to be provided in "by".`
            : [
                Error,
                'Field ',
                P,
                ` in "having" needs to be provided in "by"`,
              ]
        }[HavingFields]
      : 'take' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "take", you also need to provide "orderBy"'
      : 'skip' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "skip", you also need to provide "orderBy"'
      : ByValid extends True
      ? {}
      : {
          [P in OrderFields]: P extends ByFields
            ? never
            : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
        }[OrderFields]
    >(args: SubsetIntersection<T, PacksGroupByArgs, OrderByArg> & InputErrors): {} extends InputErrors ? GetPacksGroupByPayload<T> : Prisma.PrismaPromise<InputErrors>
  /**
   * Fields of the Packs model
   */
  readonly fields: PacksFieldRefs;
  }

  /**
   * The delegate class that acts as a "Promise-like" for Packs.
   * Why is this prefixed with `Prisma__`?
   * Because we want to prevent naming conflicts as mentioned in
   * https://github.com/prisma/prisma-client-js/issues/707
   */
  export interface Prisma__PacksClient<T, Null = never, ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs, GlobalOmitOptions = {}> extends Prisma.PrismaPromise<T> {
    readonly [Symbol.toStringTag]: "PrismaPromise"
    Images<T extends Packs$ImagesArgs<ExtArgs> = {}>(args?: Subset<T, Packs$ImagesArgs<ExtArgs>>): Prisma.PrismaPromise<$Result.GetResult<Prisma.$ImagesPayload<ExtArgs>, T, "findMany", GlobalOmitOptions> | Null>
    /**
     * Attaches callbacks for the resolution and/or rejection of the Promise.
     * @param onfulfilled The callback to execute when the Promise is resolved.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of which ever callback is executed.
     */
    then<TResult1 = T, TResult2 = never>(onfulfilled?: ((value: T) => TResult1 | PromiseLike<TResult1>) | undefined | null, onrejected?: ((reason: any) => TResult2 | PromiseLike<TResult2>) | undefined | null): $Utils.JsPromise<TResult1 | TResult2>
    /**
     * Attaches a callback for only the rejection of the Promise.
     * @param onrejected The callback to execute when the Promise is rejected.
     * @returns A Promise for the completion of the callback.
     */
    catch<TResult = never>(onrejected?: ((reason: any) => TResult | PromiseLike<TResult>) | undefined | null): $Utils.JsPromise<T | TResult>
    /**
     * Attaches a callback that is invoked when the Promise is settled (fulfilled or rejected). The
     * resolved value cannot be modified from the callback.
     * @param onfinally The callback to execute when the Promise is settled (fulfilled or rejected).
     * @returns A Promise for the completion of the callback.
     */
    finally(onfinally?: (() => void) | undefined | null): $Utils.JsPromise<T>
  }




  /**
   * Fields of the Packs model
   */ 
  interface PacksFieldRefs {
    readonly id: FieldRef<"Packs", 'Int'>
    readonly pid: FieldRef<"Packs", 'String'>
    readonly name: FieldRef<"Packs", 'String'>
    readonly description: FieldRef<"Packs", 'String'>
    readonly pack_prompts: FieldRef<"Packs", 'String'>
    readonly credits: FieldRef<"Packs", 'Int'>
    readonly amount: FieldRef<"Packs", 'Int'>
    readonly image_url: FieldRef<"Packs", 'String'>
    readonly created_at: FieldRef<"Packs", 'DateTime'>
    readonly updated_at: FieldRef<"Packs", 'DateTime'>
  }
    

  // Custom InputTypes
  /**
   * Packs findUnique
   */
  export type PacksFindUniqueArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
    /**
     * Filter, which Packs to fetch.
     */
    where: PacksWhereUniqueInput
  }

  /**
   * Packs findUniqueOrThrow
   */
  export type PacksFindUniqueOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
    /**
     * Filter, which Packs to fetch.
     */
    where: PacksWhereUniqueInput
  }

  /**
   * Packs findFirst
   */
  export type PacksFindFirstArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
    /**
     * Filter, which Packs to fetch.
     */
    where?: PacksWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Packs to fetch.
     */
    orderBy?: PacksOrderByWithRelationInput | PacksOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for Packs.
     */
    cursor?: PacksWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Packs from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Packs.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of Packs.
     */
    distinct?: PacksScalarFieldEnum | PacksScalarFieldEnum[]
  }

  /**
   * Packs findFirstOrThrow
   */
  export type PacksFindFirstOrThrowArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
    /**
     * Filter, which Packs to fetch.
     */
    where?: PacksWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Packs to fetch.
     */
    orderBy?: PacksOrderByWithRelationInput | PacksOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for searching for Packs.
     */
    cursor?: PacksWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Packs from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Packs.
     */
    skip?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/distinct Distinct Docs}
     * 
     * Filter by unique combinations of Packs.
     */
    distinct?: PacksScalarFieldEnum | PacksScalarFieldEnum[]
  }

  /**
   * Packs findMany
   */
  export type PacksFindManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
    /**
     * Filter, which Packs to fetch.
     */
    where?: PacksWhereInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/sorting Sorting Docs}
     * 
     * Determine the order of Packs to fetch.
     */
    orderBy?: PacksOrderByWithRelationInput | PacksOrderByWithRelationInput[]
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination#cursor-based-pagination Cursor Docs}
     * 
     * Sets the position for listing Packs.
     */
    cursor?: PacksWhereUniqueInput
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Take `±n` Packs from the position of the cursor.
     */
    take?: number
    /**
     * {@link https://www.prisma.io/docs/concepts/components/prisma-client/pagination Pagination Docs}
     * 
     * Skip the first `n` Packs.
     */
    skip?: number
    distinct?: PacksScalarFieldEnum | PacksScalarFieldEnum[]
  }

  /**
   * Packs create
   */
  export type PacksCreateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
    /**
     * The data needed to create a Packs.
     */
    data: XOR<PacksCreateInput, PacksUncheckedCreateInput>
  }

  /**
   * Packs createMany
   */
  export type PacksCreateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to create many Packs.
     */
    data: PacksCreateManyInput | PacksCreateManyInput[]
    skipDuplicates?: boolean
  }

  /**
   * Packs createManyAndReturn
   */
  export type PacksCreateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelectCreateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * The data used to create many Packs.
     */
    data: PacksCreateManyInput | PacksCreateManyInput[]
    skipDuplicates?: boolean
  }

  /**
   * Packs update
   */
  export type PacksUpdateArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
    /**
     * The data needed to update a Packs.
     */
    data: XOR<PacksUpdateInput, PacksUncheckedUpdateInput>
    /**
     * Choose, which Packs to update.
     */
    where: PacksWhereUniqueInput
  }

  /**
   * Packs updateMany
   */
  export type PacksUpdateManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * The data used to update Packs.
     */
    data: XOR<PacksUpdateManyMutationInput, PacksUncheckedUpdateManyInput>
    /**
     * Filter which Packs to update
     */
    where?: PacksWhereInput
    /**
     * Limit how many Packs to update.
     */
    limit?: number
  }

  /**
   * Packs updateManyAndReturn
   */
  export type PacksUpdateManyAndReturnArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelectUpdateManyAndReturn<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * The data used to update Packs.
     */
    data: XOR<PacksUpdateManyMutationInput, PacksUncheckedUpdateManyInput>
    /**
     * Filter which Packs to update
     */
    where?: PacksWhereInput
    /**
     * Limit how many Packs to update.
     */
    limit?: number
  }

  /**
   * Packs upsert
   */
  export type PacksUpsertArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
    /**
     * The filter to search for the Packs to update in case it exists.
     */
    where: PacksWhereUniqueInput
    /**
     * In case the Packs found by the `where` argument doesn't exist, create a new Packs with this data.
     */
    create: XOR<PacksCreateInput, PacksUncheckedCreateInput>
    /**
     * In case the Packs was found with the provided `where` argument, update it with this data.
     */
    update: XOR<PacksUpdateInput, PacksUncheckedUpdateInput>
  }

  /**
   * Packs delete
   */
  export type PacksDeleteArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
    /**
     * Filter which Packs to delete.
     */
    where: PacksWhereUniqueInput
  }

  /**
   * Packs deleteMany
   */
  export type PacksDeleteManyArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Filter which Packs to delete
     */
    where?: PacksWhereInput
    /**
     * Limit how many Packs to delete.
     */
    limit?: number
  }

  /**
   * Packs.Images
   */
  export type Packs$ImagesArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Images
     */
    select?: ImagesSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Images
     */
    omit?: ImagesOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: ImagesInclude<ExtArgs> | null
    where?: ImagesWhereInput
    orderBy?: ImagesOrderByWithRelationInput | ImagesOrderByWithRelationInput[]
    cursor?: ImagesWhereUniqueInput
    take?: number
    skip?: number
    distinct?: ImagesScalarFieldEnum | ImagesScalarFieldEnum[]
  }

  /**
   * Packs without action
   */
  export type PacksDefaultArgs<ExtArgs extends $Extensions.InternalArgs = $Extensions.DefaultArgs> = {
    /**
     * Select specific fields to fetch from the Packs
     */
    select?: PacksSelect<ExtArgs> | null
    /**
     * Omit specific fields from the Packs
     */
    omit?: PacksOmit<ExtArgs> | null
    /**
     * Choose, which related nodes to fetch as well
     */
    include?: PacksInclude<ExtArgs> | null
  }


  /**
   * Enums
   */

  export const TransactionIsolationLevel: {
    ReadUncommitted: 'ReadUncommitted',
    ReadCommitted: 'ReadCommitted',
    RepeatableRead: 'RepeatableRead',
    Serializable: 'Serializable'
  };

  export type TransactionIsolationLevel = (typeof TransactionIsolationLevel)[keyof typeof TransactionIsolationLevel]


  export const UsersScalarFieldEnum: {
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

  export type UsersScalarFieldEnum = (typeof UsersScalarFieldEnum)[keyof typeof UsersScalarFieldEnum]


  export const TrainingModelsScalarFieldEnum: {
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

  export type TrainingModelsScalarFieldEnum = (typeof TrainingModelsScalarFieldEnum)[keyof typeof TrainingModelsScalarFieldEnum]


  export const UserCreditsScalarFieldEnum: {
    id: 'id',
    pid: 'pid',
    user_id: 'user_id',
    credit_amount: 'credit_amount',
    model_amount: 'model_amount',
    created_at: 'created_at',
    updated_at: 'updated_at'
  };

  export type UserCreditsScalarFieldEnum = (typeof UserCreditsScalarFieldEnum)[keyof typeof UserCreditsScalarFieldEnum]


  export const ImagesScalarFieldEnum: {
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

  export type ImagesScalarFieldEnum = (typeof ImagesScalarFieldEnum)[keyof typeof ImagesScalarFieldEnum]


  export const TransactionsScalarFieldEnum: {
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

  export type TransactionsScalarFieldEnum = (typeof TransactionsScalarFieldEnum)[keyof typeof TransactionsScalarFieldEnum]


  export const PacksScalarFieldEnum: {
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

  export type PacksScalarFieldEnum = (typeof PacksScalarFieldEnum)[keyof typeof PacksScalarFieldEnum]


  export const SortOrder: {
    asc: 'asc',
    desc: 'desc'
  };

  export type SortOrder = (typeof SortOrder)[keyof typeof SortOrder]


  export const NullableJsonNullValueInput: {
    DbNull: typeof DbNull,
    JsonNull: typeof JsonNull
  };

  export type NullableJsonNullValueInput = (typeof NullableJsonNullValueInput)[keyof typeof NullableJsonNullValueInput]


  export const QueryMode: {
    default: 'default',
    insensitive: 'insensitive'
  };

  export type QueryMode = (typeof QueryMode)[keyof typeof QueryMode]


  export const NullsOrder: {
    first: 'first',
    last: 'last'
  };

  export type NullsOrder = (typeof NullsOrder)[keyof typeof NullsOrder]


  export const JsonNullValueFilter: {
    DbNull: typeof DbNull,
    JsonNull: typeof JsonNull,
    AnyNull: typeof AnyNull
  };

  export type JsonNullValueFilter = (typeof JsonNullValueFilter)[keyof typeof JsonNullValueFilter]


  /**
   * Field references 
   */


  /**
   * Reference to a field of type 'Int'
   */
  export type IntFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Int'>
    


  /**
   * Reference to a field of type 'Int[]'
   */
  export type ListIntFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Int[]'>
    


  /**
   * Reference to a field of type 'String'
   */
  export type StringFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'String'>
    


  /**
   * Reference to a field of type 'String[]'
   */
  export type ListStringFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'String[]'>
    


  /**
   * Reference to a field of type 'DateTime'
   */
  export type DateTimeFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'DateTime'>
    


  /**
   * Reference to a field of type 'DateTime[]'
   */
  export type ListDateTimeFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'DateTime[]'>
    


  /**
   * Reference to a field of type 'Sex'
   */
  export type EnumSexFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Sex'>
    


  /**
   * Reference to a field of type 'Sex[]'
   */
  export type ListEnumSexFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Sex[]'>
    


  /**
   * Reference to a field of type 'Ethnicity'
   */
  export type EnumEthnicityFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Ethnicity'>
    


  /**
   * Reference to a field of type 'Ethnicity[]'
   */
  export type ListEnumEthnicityFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Ethnicity[]'>
    


  /**
   * Reference to a field of type 'BasedOn'
   */
  export type EnumBasedOnFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'BasedOn'>
    


  /**
   * Reference to a field of type 'BasedOn[]'
   */
  export type ListEnumBasedOnFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'BasedOn[]'>
    


  /**
   * Reference to a field of type 'EyeColor'
   */
  export type EnumEyeColorFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'EyeColor'>
    


  /**
   * Reference to a field of type 'EyeColor[]'
   */
  export type ListEnumEyeColorFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'EyeColor[]'>
    


  /**
   * Reference to a field of type 'Boolean'
   */
  export type BooleanFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Boolean'>
    


  /**
   * Reference to a field of type 'Status'
   */
  export type EnumStatusFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Status'>
    


  /**
   * Reference to a field of type 'Status[]'
   */
  export type ListEnumStatusFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Status[]'>
    


  /**
   * Reference to a field of type 'Json'
   */
  export type JsonFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Json'>
    


  /**
   * Reference to a field of type 'QueryMode'
   */
  export type EnumQueryModeFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'QueryMode'>
    


  /**
   * Reference to a field of type 'ImageFormat'
   */
  export type EnumImageFormatFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'ImageFormat'>
    


  /**
   * Reference to a field of type 'ImageFormat[]'
   */
  export type ListEnumImageFormatFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'ImageFormat[]'>
    


  /**
   * Reference to a field of type 'ImageSize'
   */
  export type EnumImageSizeFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'ImageSize'>
    


  /**
   * Reference to a field of type 'ImageSize[]'
   */
  export type ListEnumImageSizeFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'ImageSize[]'>
    


  /**
   * Reference to a field of type 'Float'
   */
  export type FloatFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Float'>
    


  /**
   * Reference to a field of type 'Float[]'
   */
  export type ListFloatFieldRefInput<$PrismaModel> = FieldRefInputType<$PrismaModel, 'Float[]'>
    
  /**
   * Deep Input Types
   */


  export type UsersWhereInput = {
    AND?: UsersWhereInput | UsersWhereInput[]
    OR?: UsersWhereInput[]
    NOT?: UsersWhereInput | UsersWhereInput[]
    id?: IntFilter<"Users"> | number
    pid?: UuidFilter<"Users"> | string
    email?: StringFilter<"Users"> | string
    password?: StringFilter<"Users"> | string
    api_key?: StringFilter<"Users"> | string
    name?: StringFilter<"Users"> | string
    reset_token?: StringNullableFilter<"Users"> | string | null
    reset_sent_at?: DateTimeNullableFilter<"Users"> | Date | string | null
    email_verification_token?: StringNullableFilter<"Users"> | string | null
    email_verification_sent_at?: DateTimeNullableFilter<"Users"> | Date | string | null
    email_verified_at?: DateTimeNullableFilter<"Users"> | Date | string | null
    magicLink_token?: StringNullableFilter<"Users"> | string | null
    magicLink_expiration?: DateTimeNullableFilter<"Users"> | Date | string | null
    TrainingModels?: TrainingModelsListRelationFilter
    UserCredits?: UserCreditsListRelationFilter
    Images?: ImagesListRelationFilter
    Transactions?: TransactionsListRelationFilter
  }

  export type UsersOrderByWithRelationInput = {
    id?: SortOrder
    pid?: SortOrder
    email?: SortOrder
    password?: SortOrder
    api_key?: SortOrder
    name?: SortOrder
    reset_token?: SortOrderInput | SortOrder
    reset_sent_at?: SortOrderInput | SortOrder
    email_verification_token?: SortOrderInput | SortOrder
    email_verification_sent_at?: SortOrderInput | SortOrder
    email_verified_at?: SortOrderInput | SortOrder
    magicLink_token?: SortOrderInput | SortOrder
    magicLink_expiration?: SortOrderInput | SortOrder
    TrainingModels?: TrainingModelsOrderByRelationAggregateInput
    UserCredits?: UserCreditsOrderByRelationAggregateInput
    Images?: ImagesOrderByRelationAggregateInput
    Transactions?: TransactionsOrderByRelationAggregateInput
  }

  export type UsersWhereUniqueInput = Prisma.AtLeast<{
    id?: number
    pid?: string
    email?: string
    api_key?: string
    AND?: UsersWhereInput | UsersWhereInput[]
    OR?: UsersWhereInput[]
    NOT?: UsersWhereInput | UsersWhereInput[]
    password?: StringFilter<"Users"> | string
    name?: StringFilter<"Users"> | string
    reset_token?: StringNullableFilter<"Users"> | string | null
    reset_sent_at?: DateTimeNullableFilter<"Users"> | Date | string | null
    email_verification_token?: StringNullableFilter<"Users"> | string | null
    email_verification_sent_at?: DateTimeNullableFilter<"Users"> | Date | string | null
    email_verified_at?: DateTimeNullableFilter<"Users"> | Date | string | null
    magicLink_token?: StringNullableFilter<"Users"> | string | null
    magicLink_expiration?: DateTimeNullableFilter<"Users"> | Date | string | null
    TrainingModels?: TrainingModelsListRelationFilter
    UserCredits?: UserCreditsListRelationFilter
    Images?: ImagesListRelationFilter
    Transactions?: TransactionsListRelationFilter
  }, "id" | "pid" | "email" | "api_key">

  export type UsersOrderByWithAggregationInput = {
    id?: SortOrder
    pid?: SortOrder
    email?: SortOrder
    password?: SortOrder
    api_key?: SortOrder
    name?: SortOrder
    reset_token?: SortOrderInput | SortOrder
    reset_sent_at?: SortOrderInput | SortOrder
    email_verification_token?: SortOrderInput | SortOrder
    email_verification_sent_at?: SortOrderInput | SortOrder
    email_verified_at?: SortOrderInput | SortOrder
    magicLink_token?: SortOrderInput | SortOrder
    magicLink_expiration?: SortOrderInput | SortOrder
    _count?: UsersCountOrderByAggregateInput
    _avg?: UsersAvgOrderByAggregateInput
    _max?: UsersMaxOrderByAggregateInput
    _min?: UsersMinOrderByAggregateInput
    _sum?: UsersSumOrderByAggregateInput
  }

  export type UsersScalarWhereWithAggregatesInput = {
    AND?: UsersScalarWhereWithAggregatesInput | UsersScalarWhereWithAggregatesInput[]
    OR?: UsersScalarWhereWithAggregatesInput[]
    NOT?: UsersScalarWhereWithAggregatesInput | UsersScalarWhereWithAggregatesInput[]
    id?: IntWithAggregatesFilter<"Users"> | number
    pid?: UuidWithAggregatesFilter<"Users"> | string
    email?: StringWithAggregatesFilter<"Users"> | string
    password?: StringWithAggregatesFilter<"Users"> | string
    api_key?: StringWithAggregatesFilter<"Users"> | string
    name?: StringWithAggregatesFilter<"Users"> | string
    reset_token?: StringNullableWithAggregatesFilter<"Users"> | string | null
    reset_sent_at?: DateTimeNullableWithAggregatesFilter<"Users"> | Date | string | null
    email_verification_token?: StringNullableWithAggregatesFilter<"Users"> | string | null
    email_verification_sent_at?: DateTimeNullableWithAggregatesFilter<"Users"> | Date | string | null
    email_verified_at?: DateTimeNullableWithAggregatesFilter<"Users"> | Date | string | null
    magicLink_token?: StringNullableWithAggregatesFilter<"Users"> | string | null
    magicLink_expiration?: DateTimeNullableWithAggregatesFilter<"Users"> | Date | string | null
  }

  export type TrainingModelsWhereInput = {
    AND?: TrainingModelsWhereInput | TrainingModelsWhereInput[]
    OR?: TrainingModelsWhereInput[]
    NOT?: TrainingModelsWhereInput | TrainingModelsWhereInput[]
    id?: IntFilter<"TrainingModels"> | number
    pid?: UuidFilter<"TrainingModels"> | string
    user_id?: IntFilter<"TrainingModels"> | number
    name?: StringFilter<"TrainingModels"> | string
    age?: IntFilter<"TrainingModels"> | number
    sex?: EnumSexFilter<"TrainingModels"> | $Enums.Sex
    ethnicity?: EnumEthnicityFilter<"TrainingModels"> | $Enums.Ethnicity
    basedOn?: EnumBasedOnFilter<"TrainingModels"> | $Enums.BasedOn
    eye_color?: EnumEyeColorFilter<"TrainingModels"> | $Enums.EyeColor
    bald?: BoolFilter<"TrainingModels"> | boolean
    steps?: IntFilter<"TrainingModels"> | number
    create_mask?: BoolFilter<"TrainingModels"> | boolean
    is_style?: BoolFilter<"TrainingModels"> | boolean
    trigger_word?: StringFilter<"TrainingModels"> | string
    tensor_path?: StringNullableFilter<"TrainingModels"> | string | null
    thumbnail?: StringNullableFilter<"TrainingModels"> | string | null
    training_status?: EnumStatusFilter<"TrainingModels"> | $Enums.Status
    fal_output?: JsonNullableFilter<"TrainingModels">
    training_images?: JsonNullableFilter<"TrainingModels">
    fal_ai_request_id?: StringNullableFilter<"TrainingModels"> | string | null
    s3_key?: StringFilter<"TrainingModels"> | string
    is_verified?: BoolFilter<"TrainingModels"> | boolean
    deleted_at?: DateTimeNullableFilter<"TrainingModels"> | Date | string | null
    created_at?: DateTimeFilter<"TrainingModels"> | Date | string
    updated_at?: DateTimeFilter<"TrainingModels"> | Date | string
    user?: XOR<UsersScalarRelationFilter, UsersWhereInput>
    Images?: ImagesListRelationFilter
  }

  export type TrainingModelsOrderByWithRelationInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    name?: SortOrder
    age?: SortOrder
    sex?: SortOrder
    ethnicity?: SortOrder
    basedOn?: SortOrder
    eye_color?: SortOrder
    bald?: SortOrder
    steps?: SortOrder
    create_mask?: SortOrder
    is_style?: SortOrder
    trigger_word?: SortOrder
    tensor_path?: SortOrderInput | SortOrder
    thumbnail?: SortOrderInput | SortOrder
    training_status?: SortOrder
    fal_output?: SortOrderInput | SortOrder
    training_images?: SortOrderInput | SortOrder
    fal_ai_request_id?: SortOrderInput | SortOrder
    s3_key?: SortOrder
    is_verified?: SortOrder
    deleted_at?: SortOrderInput | SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
    user?: UsersOrderByWithRelationInput
    Images?: ImagesOrderByRelationAggregateInput
  }

  export type TrainingModelsWhereUniqueInput = Prisma.AtLeast<{
    id?: number
    pid?: string
    AND?: TrainingModelsWhereInput | TrainingModelsWhereInput[]
    OR?: TrainingModelsWhereInput[]
    NOT?: TrainingModelsWhereInput | TrainingModelsWhereInput[]
    user_id?: IntFilter<"TrainingModels"> | number
    name?: StringFilter<"TrainingModels"> | string
    age?: IntFilter<"TrainingModels"> | number
    sex?: EnumSexFilter<"TrainingModels"> | $Enums.Sex
    ethnicity?: EnumEthnicityFilter<"TrainingModels"> | $Enums.Ethnicity
    basedOn?: EnumBasedOnFilter<"TrainingModels"> | $Enums.BasedOn
    eye_color?: EnumEyeColorFilter<"TrainingModels"> | $Enums.EyeColor
    bald?: BoolFilter<"TrainingModels"> | boolean
    steps?: IntFilter<"TrainingModels"> | number
    create_mask?: BoolFilter<"TrainingModels"> | boolean
    is_style?: BoolFilter<"TrainingModels"> | boolean
    trigger_word?: StringFilter<"TrainingModels"> | string
    tensor_path?: StringNullableFilter<"TrainingModels"> | string | null
    thumbnail?: StringNullableFilter<"TrainingModels"> | string | null
    training_status?: EnumStatusFilter<"TrainingModels"> | $Enums.Status
    fal_output?: JsonNullableFilter<"TrainingModels">
    training_images?: JsonNullableFilter<"TrainingModels">
    fal_ai_request_id?: StringNullableFilter<"TrainingModels"> | string | null
    s3_key?: StringFilter<"TrainingModels"> | string
    is_verified?: BoolFilter<"TrainingModels"> | boolean
    deleted_at?: DateTimeNullableFilter<"TrainingModels"> | Date | string | null
    created_at?: DateTimeFilter<"TrainingModels"> | Date | string
    updated_at?: DateTimeFilter<"TrainingModels"> | Date | string
    user?: XOR<UsersScalarRelationFilter, UsersWhereInput>
    Images?: ImagesListRelationFilter
  }, "id" | "pid">

  export type TrainingModelsOrderByWithAggregationInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    name?: SortOrder
    age?: SortOrder
    sex?: SortOrder
    ethnicity?: SortOrder
    basedOn?: SortOrder
    eye_color?: SortOrder
    bald?: SortOrder
    steps?: SortOrder
    create_mask?: SortOrder
    is_style?: SortOrder
    trigger_word?: SortOrder
    tensor_path?: SortOrderInput | SortOrder
    thumbnail?: SortOrderInput | SortOrder
    training_status?: SortOrder
    fal_output?: SortOrderInput | SortOrder
    training_images?: SortOrderInput | SortOrder
    fal_ai_request_id?: SortOrderInput | SortOrder
    s3_key?: SortOrder
    is_verified?: SortOrder
    deleted_at?: SortOrderInput | SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
    _count?: TrainingModelsCountOrderByAggregateInput
    _avg?: TrainingModelsAvgOrderByAggregateInput
    _max?: TrainingModelsMaxOrderByAggregateInput
    _min?: TrainingModelsMinOrderByAggregateInput
    _sum?: TrainingModelsSumOrderByAggregateInput
  }

  export type TrainingModelsScalarWhereWithAggregatesInput = {
    AND?: TrainingModelsScalarWhereWithAggregatesInput | TrainingModelsScalarWhereWithAggregatesInput[]
    OR?: TrainingModelsScalarWhereWithAggregatesInput[]
    NOT?: TrainingModelsScalarWhereWithAggregatesInput | TrainingModelsScalarWhereWithAggregatesInput[]
    id?: IntWithAggregatesFilter<"TrainingModels"> | number
    pid?: UuidWithAggregatesFilter<"TrainingModels"> | string
    user_id?: IntWithAggregatesFilter<"TrainingModels"> | number
    name?: StringWithAggregatesFilter<"TrainingModels"> | string
    age?: IntWithAggregatesFilter<"TrainingModels"> | number
    sex?: EnumSexWithAggregatesFilter<"TrainingModels"> | $Enums.Sex
    ethnicity?: EnumEthnicityWithAggregatesFilter<"TrainingModels"> | $Enums.Ethnicity
    basedOn?: EnumBasedOnWithAggregatesFilter<"TrainingModels"> | $Enums.BasedOn
    eye_color?: EnumEyeColorWithAggregatesFilter<"TrainingModels"> | $Enums.EyeColor
    bald?: BoolWithAggregatesFilter<"TrainingModels"> | boolean
    steps?: IntWithAggregatesFilter<"TrainingModels"> | number
    create_mask?: BoolWithAggregatesFilter<"TrainingModels"> | boolean
    is_style?: BoolWithAggregatesFilter<"TrainingModels"> | boolean
    trigger_word?: StringWithAggregatesFilter<"TrainingModels"> | string
    tensor_path?: StringNullableWithAggregatesFilter<"TrainingModels"> | string | null
    thumbnail?: StringNullableWithAggregatesFilter<"TrainingModels"> | string | null
    training_status?: EnumStatusWithAggregatesFilter<"TrainingModels"> | $Enums.Status
    fal_output?: JsonNullableWithAggregatesFilter<"TrainingModels">
    training_images?: JsonNullableWithAggregatesFilter<"TrainingModels">
    fal_ai_request_id?: StringNullableWithAggregatesFilter<"TrainingModels"> | string | null
    s3_key?: StringWithAggregatesFilter<"TrainingModels"> | string
    is_verified?: BoolWithAggregatesFilter<"TrainingModels"> | boolean
    deleted_at?: DateTimeNullableWithAggregatesFilter<"TrainingModels"> | Date | string | null
    created_at?: DateTimeWithAggregatesFilter<"TrainingModels"> | Date | string
    updated_at?: DateTimeWithAggregatesFilter<"TrainingModels"> | Date | string
  }

  export type UserCreditsWhereInput = {
    AND?: UserCreditsWhereInput | UserCreditsWhereInput[]
    OR?: UserCreditsWhereInput[]
    NOT?: UserCreditsWhereInput | UserCreditsWhereInput[]
    id?: IntFilter<"UserCredits"> | number
    pid?: UuidFilter<"UserCredits"> | string
    user_id?: IntFilter<"UserCredits"> | number
    credit_amount?: IntFilter<"UserCredits"> | number
    model_amount?: IntFilter<"UserCredits"> | number
    created_at?: DateTimeFilter<"UserCredits"> | Date | string
    updated_at?: DateTimeFilter<"UserCredits"> | Date | string
    user?: XOR<UsersScalarRelationFilter, UsersWhereInput>
  }

  export type UserCreditsOrderByWithRelationInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
    user?: UsersOrderByWithRelationInput
  }

  export type UserCreditsWhereUniqueInput = Prisma.AtLeast<{
    id?: number
    pid?: string
    user_id?: number
    AND?: UserCreditsWhereInput | UserCreditsWhereInput[]
    OR?: UserCreditsWhereInput[]
    NOT?: UserCreditsWhereInput | UserCreditsWhereInput[]
    credit_amount?: IntFilter<"UserCredits"> | number
    model_amount?: IntFilter<"UserCredits"> | number
    created_at?: DateTimeFilter<"UserCredits"> | Date | string
    updated_at?: DateTimeFilter<"UserCredits"> | Date | string
    user?: XOR<UsersScalarRelationFilter, UsersWhereInput>
  }, "id" | "pid" | "user_id">

  export type UserCreditsOrderByWithAggregationInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
    _count?: UserCreditsCountOrderByAggregateInput
    _avg?: UserCreditsAvgOrderByAggregateInput
    _max?: UserCreditsMaxOrderByAggregateInput
    _min?: UserCreditsMinOrderByAggregateInput
    _sum?: UserCreditsSumOrderByAggregateInput
  }

  export type UserCreditsScalarWhereWithAggregatesInput = {
    AND?: UserCreditsScalarWhereWithAggregatesInput | UserCreditsScalarWhereWithAggregatesInput[]
    OR?: UserCreditsScalarWhereWithAggregatesInput[]
    NOT?: UserCreditsScalarWhereWithAggregatesInput | UserCreditsScalarWhereWithAggregatesInput[]
    id?: IntWithAggregatesFilter<"UserCredits"> | number
    pid?: UuidWithAggregatesFilter<"UserCredits"> | string
    user_id?: IntWithAggregatesFilter<"UserCredits"> | number
    credit_amount?: IntWithAggregatesFilter<"UserCredits"> | number
    model_amount?: IntWithAggregatesFilter<"UserCredits"> | number
    created_at?: DateTimeWithAggregatesFilter<"UserCredits"> | Date | string
    updated_at?: DateTimeWithAggregatesFilter<"UserCredits"> | Date | string
  }

  export type ImagesWhereInput = {
    AND?: ImagesWhereInput | ImagesWhereInput[]
    OR?: ImagesWhereInput[]
    NOT?: ImagesWhereInput | ImagesWhereInput[]
    id?: IntFilter<"Images"> | number
    pid?: UuidFilter<"Images"> | string
    user_id?: IntFilter<"Images"> | number
    training_model_id?: IntFilter<"Images"> | number
    pack_id?: IntNullableFilter<"Images"> | number | null
    user_prompt?: StringFilter<"Images"> | string
    sys_prompt?: StringFilter<"Images"> | string
    alt?: StringFilter<"Images"> | string
    num_inference_steps?: IntFilter<"Images"> | number
    content_type?: EnumImageFormatFilter<"Images"> | $Enums.ImageFormat
    status?: EnumStatusFilter<"Images"> | $Enums.Status
    image_size?: EnumImageSizeFilter<"Images"> | $Enums.ImageSize
    fal_ai_request_id?: StringNullableFilter<"Images"> | string | null
    width?: IntNullableFilter<"Images"> | number | null
    height?: IntNullableFilter<"Images"> | number | null
    image_url?: StringNullableFilter<"Images"> | string | null
    image_url_s3?: StringNullableFilter<"Images"> | string | null
    is_favorite?: BoolFilter<"Images"> | boolean
    deleted_at?: DateTimeNullableFilter<"Images"> | Date | string | null
    created_at?: DateTimeFilter<"Images"> | Date | string
    updated_at?: DateTimeFilter<"Images"> | Date | string
    user?: XOR<UsersScalarRelationFilter, UsersWhereInput>
    training_model?: XOR<TrainingModelsScalarRelationFilter, TrainingModelsWhereInput>
    pack?: XOR<PacksNullableScalarRelationFilter, PacksWhereInput> | null
  }

  export type ImagesOrderByWithRelationInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    training_model_id?: SortOrder
    pack_id?: SortOrderInput | SortOrder
    user_prompt?: SortOrder
    sys_prompt?: SortOrder
    alt?: SortOrder
    num_inference_steps?: SortOrder
    content_type?: SortOrder
    status?: SortOrder
    image_size?: SortOrder
    fal_ai_request_id?: SortOrderInput | SortOrder
    width?: SortOrderInput | SortOrder
    height?: SortOrderInput | SortOrder
    image_url?: SortOrderInput | SortOrder
    image_url_s3?: SortOrderInput | SortOrder
    is_favorite?: SortOrder
    deleted_at?: SortOrderInput | SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
    user?: UsersOrderByWithRelationInput
    training_model?: TrainingModelsOrderByWithRelationInput
    pack?: PacksOrderByWithRelationInput
  }

  export type ImagesWhereUniqueInput = Prisma.AtLeast<{
    id?: number
    pid?: string
    AND?: ImagesWhereInput | ImagesWhereInput[]
    OR?: ImagesWhereInput[]
    NOT?: ImagesWhereInput | ImagesWhereInput[]
    user_id?: IntFilter<"Images"> | number
    training_model_id?: IntFilter<"Images"> | number
    pack_id?: IntNullableFilter<"Images"> | number | null
    user_prompt?: StringFilter<"Images"> | string
    sys_prompt?: StringFilter<"Images"> | string
    alt?: StringFilter<"Images"> | string
    num_inference_steps?: IntFilter<"Images"> | number
    content_type?: EnumImageFormatFilter<"Images"> | $Enums.ImageFormat
    status?: EnumStatusFilter<"Images"> | $Enums.Status
    image_size?: EnumImageSizeFilter<"Images"> | $Enums.ImageSize
    fal_ai_request_id?: StringNullableFilter<"Images"> | string | null
    width?: IntNullableFilter<"Images"> | number | null
    height?: IntNullableFilter<"Images"> | number | null
    image_url?: StringNullableFilter<"Images"> | string | null
    image_url_s3?: StringNullableFilter<"Images"> | string | null
    is_favorite?: BoolFilter<"Images"> | boolean
    deleted_at?: DateTimeNullableFilter<"Images"> | Date | string | null
    created_at?: DateTimeFilter<"Images"> | Date | string
    updated_at?: DateTimeFilter<"Images"> | Date | string
    user?: XOR<UsersScalarRelationFilter, UsersWhereInput>
    training_model?: XOR<TrainingModelsScalarRelationFilter, TrainingModelsWhereInput>
    pack?: XOR<PacksNullableScalarRelationFilter, PacksWhereInput> | null
  }, "id" | "pid">

  export type ImagesOrderByWithAggregationInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    training_model_id?: SortOrder
    pack_id?: SortOrderInput | SortOrder
    user_prompt?: SortOrder
    sys_prompt?: SortOrder
    alt?: SortOrder
    num_inference_steps?: SortOrder
    content_type?: SortOrder
    status?: SortOrder
    image_size?: SortOrder
    fal_ai_request_id?: SortOrderInput | SortOrder
    width?: SortOrderInput | SortOrder
    height?: SortOrderInput | SortOrder
    image_url?: SortOrderInput | SortOrder
    image_url_s3?: SortOrderInput | SortOrder
    is_favorite?: SortOrder
    deleted_at?: SortOrderInput | SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
    _count?: ImagesCountOrderByAggregateInput
    _avg?: ImagesAvgOrderByAggregateInput
    _max?: ImagesMaxOrderByAggregateInput
    _min?: ImagesMinOrderByAggregateInput
    _sum?: ImagesSumOrderByAggregateInput
  }

  export type ImagesScalarWhereWithAggregatesInput = {
    AND?: ImagesScalarWhereWithAggregatesInput | ImagesScalarWhereWithAggregatesInput[]
    OR?: ImagesScalarWhereWithAggregatesInput[]
    NOT?: ImagesScalarWhereWithAggregatesInput | ImagesScalarWhereWithAggregatesInput[]
    id?: IntWithAggregatesFilter<"Images"> | number
    pid?: UuidWithAggregatesFilter<"Images"> | string
    user_id?: IntWithAggregatesFilter<"Images"> | number
    training_model_id?: IntWithAggregatesFilter<"Images"> | number
    pack_id?: IntNullableWithAggregatesFilter<"Images"> | number | null
    user_prompt?: StringWithAggregatesFilter<"Images"> | string
    sys_prompt?: StringWithAggregatesFilter<"Images"> | string
    alt?: StringWithAggregatesFilter<"Images"> | string
    num_inference_steps?: IntWithAggregatesFilter<"Images"> | number
    content_type?: EnumImageFormatWithAggregatesFilter<"Images"> | $Enums.ImageFormat
    status?: EnumStatusWithAggregatesFilter<"Images"> | $Enums.Status
    image_size?: EnumImageSizeWithAggregatesFilter<"Images"> | $Enums.ImageSize
    fal_ai_request_id?: StringNullableWithAggregatesFilter<"Images"> | string | null
    width?: IntNullableWithAggregatesFilter<"Images"> | number | null
    height?: IntNullableWithAggregatesFilter<"Images"> | number | null
    image_url?: StringNullableWithAggregatesFilter<"Images"> | string | null
    image_url_s3?: StringNullableWithAggregatesFilter<"Images"> | string | null
    is_favorite?: BoolWithAggregatesFilter<"Images"> | boolean
    deleted_at?: DateTimeNullableWithAggregatesFilter<"Images"> | Date | string | null
    created_at?: DateTimeWithAggregatesFilter<"Images"> | Date | string
    updated_at?: DateTimeWithAggregatesFilter<"Images"> | Date | string
  }

  export type TransactionsWhereInput = {
    AND?: TransactionsWhereInput | TransactionsWhereInput[]
    OR?: TransactionsWhereInput[]
    NOT?: TransactionsWhereInput | TransactionsWhereInput[]
    id?: IntFilter<"Transactions"> | number
    pid?: UuidFilter<"Transactions"> | string
    user_id?: IntFilter<"Transactions"> | number
    credit_amount?: IntFilter<"Transactions"> | number
    model_amount?: IntFilter<"Transactions"> | number
    currency?: StringFilter<"Transactions"> | string
    payment_id?: StringFilter<"Transactions"> | string
    order_id?: StringFilter<"Transactions"> | string
    plan?: StringFilter<"Transactions"> | string
    status?: EnumStatusFilter<"Transactions"> | $Enums.Status
    created_at?: DateTimeFilter<"Transactions"> | Date | string
    updated_at?: DateTimeFilter<"Transactions"> | Date | string
    user?: XOR<UsersScalarRelationFilter, UsersWhereInput>
  }

  export type TransactionsOrderByWithRelationInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
    currency?: SortOrder
    payment_id?: SortOrder
    order_id?: SortOrder
    plan?: SortOrder
    status?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
    user?: UsersOrderByWithRelationInput
  }

  export type TransactionsWhereUniqueInput = Prisma.AtLeast<{
    id?: number
    pid?: string
    AND?: TransactionsWhereInput | TransactionsWhereInput[]
    OR?: TransactionsWhereInput[]
    NOT?: TransactionsWhereInput | TransactionsWhereInput[]
    user_id?: IntFilter<"Transactions"> | number
    credit_amount?: IntFilter<"Transactions"> | number
    model_amount?: IntFilter<"Transactions"> | number
    currency?: StringFilter<"Transactions"> | string
    payment_id?: StringFilter<"Transactions"> | string
    order_id?: StringFilter<"Transactions"> | string
    plan?: StringFilter<"Transactions"> | string
    status?: EnumStatusFilter<"Transactions"> | $Enums.Status
    created_at?: DateTimeFilter<"Transactions"> | Date | string
    updated_at?: DateTimeFilter<"Transactions"> | Date | string
    user?: XOR<UsersScalarRelationFilter, UsersWhereInput>
  }, "id" | "pid">

  export type TransactionsOrderByWithAggregationInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
    currency?: SortOrder
    payment_id?: SortOrder
    order_id?: SortOrder
    plan?: SortOrder
    status?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
    _count?: TransactionsCountOrderByAggregateInput
    _avg?: TransactionsAvgOrderByAggregateInput
    _max?: TransactionsMaxOrderByAggregateInput
    _min?: TransactionsMinOrderByAggregateInput
    _sum?: TransactionsSumOrderByAggregateInput
  }

  export type TransactionsScalarWhereWithAggregatesInput = {
    AND?: TransactionsScalarWhereWithAggregatesInput | TransactionsScalarWhereWithAggregatesInput[]
    OR?: TransactionsScalarWhereWithAggregatesInput[]
    NOT?: TransactionsScalarWhereWithAggregatesInput | TransactionsScalarWhereWithAggregatesInput[]
    id?: IntWithAggregatesFilter<"Transactions"> | number
    pid?: UuidWithAggregatesFilter<"Transactions"> | string
    user_id?: IntWithAggregatesFilter<"Transactions"> | number
    credit_amount?: IntWithAggregatesFilter<"Transactions"> | number
    model_amount?: IntWithAggregatesFilter<"Transactions"> | number
    currency?: StringWithAggregatesFilter<"Transactions"> | string
    payment_id?: StringWithAggregatesFilter<"Transactions"> | string
    order_id?: StringWithAggregatesFilter<"Transactions"> | string
    plan?: StringWithAggregatesFilter<"Transactions"> | string
    status?: EnumStatusWithAggregatesFilter<"Transactions"> | $Enums.Status
    created_at?: DateTimeWithAggregatesFilter<"Transactions"> | Date | string
    updated_at?: DateTimeWithAggregatesFilter<"Transactions"> | Date | string
  }

  export type PacksWhereInput = {
    AND?: PacksWhereInput | PacksWhereInput[]
    OR?: PacksWhereInput[]
    NOT?: PacksWhereInput | PacksWhereInput[]
    id?: IntFilter<"Packs"> | number
    pid?: UuidFilter<"Packs"> | string
    name?: StringFilter<"Packs"> | string
    description?: StringFilter<"Packs"> | string
    pack_prompts?: StringFilter<"Packs"> | string
    credits?: IntFilter<"Packs"> | number
    amount?: IntFilter<"Packs"> | number
    image_url?: StringFilter<"Packs"> | string
    created_at?: DateTimeFilter<"Packs"> | Date | string
    updated_at?: DateTimeFilter<"Packs"> | Date | string
    Images?: ImagesListRelationFilter
  }

  export type PacksOrderByWithRelationInput = {
    id?: SortOrder
    pid?: SortOrder
    name?: SortOrder
    description?: SortOrder
    pack_prompts?: SortOrder
    credits?: SortOrder
    amount?: SortOrder
    image_url?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
    Images?: ImagesOrderByRelationAggregateInput
  }

  export type PacksWhereUniqueInput = Prisma.AtLeast<{
    id?: number
    pid?: string
    AND?: PacksWhereInput | PacksWhereInput[]
    OR?: PacksWhereInput[]
    NOT?: PacksWhereInput | PacksWhereInput[]
    name?: StringFilter<"Packs"> | string
    description?: StringFilter<"Packs"> | string
    pack_prompts?: StringFilter<"Packs"> | string
    credits?: IntFilter<"Packs"> | number
    amount?: IntFilter<"Packs"> | number
    image_url?: StringFilter<"Packs"> | string
    created_at?: DateTimeFilter<"Packs"> | Date | string
    updated_at?: DateTimeFilter<"Packs"> | Date | string
    Images?: ImagesListRelationFilter
  }, "id" | "pid">

  export type PacksOrderByWithAggregationInput = {
    id?: SortOrder
    pid?: SortOrder
    name?: SortOrder
    description?: SortOrder
    pack_prompts?: SortOrder
    credits?: SortOrder
    amount?: SortOrder
    image_url?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
    _count?: PacksCountOrderByAggregateInput
    _avg?: PacksAvgOrderByAggregateInput
    _max?: PacksMaxOrderByAggregateInput
    _min?: PacksMinOrderByAggregateInput
    _sum?: PacksSumOrderByAggregateInput
  }

  export type PacksScalarWhereWithAggregatesInput = {
    AND?: PacksScalarWhereWithAggregatesInput | PacksScalarWhereWithAggregatesInput[]
    OR?: PacksScalarWhereWithAggregatesInput[]
    NOT?: PacksScalarWhereWithAggregatesInput | PacksScalarWhereWithAggregatesInput[]
    id?: IntWithAggregatesFilter<"Packs"> | number
    pid?: UuidWithAggregatesFilter<"Packs"> | string
    name?: StringWithAggregatesFilter<"Packs"> | string
    description?: StringWithAggregatesFilter<"Packs"> | string
    pack_prompts?: StringWithAggregatesFilter<"Packs"> | string
    credits?: IntWithAggregatesFilter<"Packs"> | number
    amount?: IntWithAggregatesFilter<"Packs"> | number
    image_url?: StringWithAggregatesFilter<"Packs"> | string
    created_at?: DateTimeWithAggregatesFilter<"Packs"> | Date | string
    updated_at?: DateTimeWithAggregatesFilter<"Packs"> | Date | string
  }

  export type UsersCreateInput = {
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
    TrainingModels?: TrainingModelsCreateNestedManyWithoutUserInput
    UserCredits?: UserCreditsCreateNestedManyWithoutUserInput
    Images?: ImagesCreateNestedManyWithoutUserInput
    Transactions?: TransactionsCreateNestedManyWithoutUserInput
  }

  export type UsersUncheckedCreateInput = {
    id?: number
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
    TrainingModels?: TrainingModelsUncheckedCreateNestedManyWithoutUserInput
    UserCredits?: UserCreditsUncheckedCreateNestedManyWithoutUserInput
    Images?: ImagesUncheckedCreateNestedManyWithoutUserInput
    Transactions?: TransactionsUncheckedCreateNestedManyWithoutUserInput
  }

  export type UsersUpdateInput = {
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    TrainingModels?: TrainingModelsUpdateManyWithoutUserNestedInput
    UserCredits?: UserCreditsUpdateManyWithoutUserNestedInput
    Images?: ImagesUpdateManyWithoutUserNestedInput
    Transactions?: TransactionsUpdateManyWithoutUserNestedInput
  }

  export type UsersUncheckedUpdateInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    TrainingModels?: TrainingModelsUncheckedUpdateManyWithoutUserNestedInput
    UserCredits?: UserCreditsUncheckedUpdateManyWithoutUserNestedInput
    Images?: ImagesUncheckedUpdateManyWithoutUserNestedInput
    Transactions?: TransactionsUncheckedUpdateManyWithoutUserNestedInput
  }

  export type UsersCreateManyInput = {
    id?: number
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
  }

  export type UsersUpdateManyMutationInput = {
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
  }

  export type UsersUncheckedUpdateManyInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
  }

  export type TrainingModelsCreateInput = {
    pid: string
    name: string
    age: number
    sex: $Enums.Sex
    ethnicity: $Enums.Ethnicity
    basedOn: $Enums.BasedOn
    eye_color: $Enums.EyeColor
    bald?: boolean
    steps: number
    create_mask: boolean
    is_style: boolean
    trigger_word: string
    tensor_path?: string | null
    thumbnail?: string | null
    training_status: $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: string | null
    s3_key: string
    is_verified?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
    user: UsersCreateNestedOneWithoutTrainingModelsInput
    Images?: ImagesCreateNestedManyWithoutTraining_modelInput
  }

  export type TrainingModelsUncheckedCreateInput = {
    id?: number
    pid: string
    user_id: number
    name: string
    age: number
    sex: $Enums.Sex
    ethnicity: $Enums.Ethnicity
    basedOn: $Enums.BasedOn
    eye_color: $Enums.EyeColor
    bald?: boolean
    steps: number
    create_mask: boolean
    is_style: boolean
    trigger_word: string
    tensor_path?: string | null
    thumbnail?: string | null
    training_status: $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: string | null
    s3_key: string
    is_verified?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
    Images?: ImagesUncheckedCreateNestedManyWithoutTraining_modelInput
  }

  export type TrainingModelsUpdateInput = {
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    age?: IntFieldUpdateOperationsInput | number
    sex?: EnumSexFieldUpdateOperationsInput | $Enums.Sex
    ethnicity?: EnumEthnicityFieldUpdateOperationsInput | $Enums.Ethnicity
    basedOn?: EnumBasedOnFieldUpdateOperationsInput | $Enums.BasedOn
    eye_color?: EnumEyeColorFieldUpdateOperationsInput | $Enums.EyeColor
    bald?: BoolFieldUpdateOperationsInput | boolean
    steps?: IntFieldUpdateOperationsInput | number
    create_mask?: BoolFieldUpdateOperationsInput | boolean
    is_style?: BoolFieldUpdateOperationsInput | boolean
    trigger_word?: StringFieldUpdateOperationsInput | string
    tensor_path?: NullableStringFieldUpdateOperationsInput | string | null
    thumbnail?: NullableStringFieldUpdateOperationsInput | string | null
    training_status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    s3_key?: StringFieldUpdateOperationsInput | string
    is_verified?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    user?: UsersUpdateOneRequiredWithoutTrainingModelsNestedInput
    Images?: ImagesUpdateManyWithoutTraining_modelNestedInput
  }

  export type TrainingModelsUncheckedUpdateInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    name?: StringFieldUpdateOperationsInput | string
    age?: IntFieldUpdateOperationsInput | number
    sex?: EnumSexFieldUpdateOperationsInput | $Enums.Sex
    ethnicity?: EnumEthnicityFieldUpdateOperationsInput | $Enums.Ethnicity
    basedOn?: EnumBasedOnFieldUpdateOperationsInput | $Enums.BasedOn
    eye_color?: EnumEyeColorFieldUpdateOperationsInput | $Enums.EyeColor
    bald?: BoolFieldUpdateOperationsInput | boolean
    steps?: IntFieldUpdateOperationsInput | number
    create_mask?: BoolFieldUpdateOperationsInput | boolean
    is_style?: BoolFieldUpdateOperationsInput | boolean
    trigger_word?: StringFieldUpdateOperationsInput | string
    tensor_path?: NullableStringFieldUpdateOperationsInput | string | null
    thumbnail?: NullableStringFieldUpdateOperationsInput | string | null
    training_status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    s3_key?: StringFieldUpdateOperationsInput | string
    is_verified?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    Images?: ImagesUncheckedUpdateManyWithoutTraining_modelNestedInput
  }

  export type TrainingModelsCreateManyInput = {
    id?: number
    pid: string
    user_id: number
    name: string
    age: number
    sex: $Enums.Sex
    ethnicity: $Enums.Ethnicity
    basedOn: $Enums.BasedOn
    eye_color: $Enums.EyeColor
    bald?: boolean
    steps: number
    create_mask: boolean
    is_style: boolean
    trigger_word: string
    tensor_path?: string | null
    thumbnail?: string | null
    training_status: $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: string | null
    s3_key: string
    is_verified?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type TrainingModelsUpdateManyMutationInput = {
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    age?: IntFieldUpdateOperationsInput | number
    sex?: EnumSexFieldUpdateOperationsInput | $Enums.Sex
    ethnicity?: EnumEthnicityFieldUpdateOperationsInput | $Enums.Ethnicity
    basedOn?: EnumBasedOnFieldUpdateOperationsInput | $Enums.BasedOn
    eye_color?: EnumEyeColorFieldUpdateOperationsInput | $Enums.EyeColor
    bald?: BoolFieldUpdateOperationsInput | boolean
    steps?: IntFieldUpdateOperationsInput | number
    create_mask?: BoolFieldUpdateOperationsInput | boolean
    is_style?: BoolFieldUpdateOperationsInput | boolean
    trigger_word?: StringFieldUpdateOperationsInput | string
    tensor_path?: NullableStringFieldUpdateOperationsInput | string | null
    thumbnail?: NullableStringFieldUpdateOperationsInput | string | null
    training_status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    s3_key?: StringFieldUpdateOperationsInput | string
    is_verified?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type TrainingModelsUncheckedUpdateManyInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    name?: StringFieldUpdateOperationsInput | string
    age?: IntFieldUpdateOperationsInput | number
    sex?: EnumSexFieldUpdateOperationsInput | $Enums.Sex
    ethnicity?: EnumEthnicityFieldUpdateOperationsInput | $Enums.Ethnicity
    basedOn?: EnumBasedOnFieldUpdateOperationsInput | $Enums.BasedOn
    eye_color?: EnumEyeColorFieldUpdateOperationsInput | $Enums.EyeColor
    bald?: BoolFieldUpdateOperationsInput | boolean
    steps?: IntFieldUpdateOperationsInput | number
    create_mask?: BoolFieldUpdateOperationsInput | boolean
    is_style?: BoolFieldUpdateOperationsInput | boolean
    trigger_word?: StringFieldUpdateOperationsInput | string
    tensor_path?: NullableStringFieldUpdateOperationsInput | string | null
    thumbnail?: NullableStringFieldUpdateOperationsInput | string | null
    training_status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    s3_key?: StringFieldUpdateOperationsInput | string
    is_verified?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type UserCreditsCreateInput = {
    pid: string
    credit_amount?: number
    model_amount?: number
    created_at?: Date | string
    updated_at?: Date | string
    user: UsersCreateNestedOneWithoutUserCreditsInput
  }

  export type UserCreditsUncheckedCreateInput = {
    id?: number
    pid: string
    user_id: number
    credit_amount?: number
    model_amount?: number
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type UserCreditsUpdateInput = {
    pid?: StringFieldUpdateOperationsInput | string
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    user?: UsersUpdateOneRequiredWithoutUserCreditsNestedInput
  }

  export type UserCreditsUncheckedUpdateInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type UserCreditsCreateManyInput = {
    id?: number
    pid: string
    user_id: number
    credit_amount?: number
    model_amount?: number
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type UserCreditsUpdateManyMutationInput = {
    pid?: StringFieldUpdateOperationsInput | string
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type UserCreditsUncheckedUpdateManyInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type ImagesCreateInput = {
    pid: string
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
    user: UsersCreateNestedOneWithoutImagesInput
    training_model: TrainingModelsCreateNestedOneWithoutImagesInput
    pack?: PacksCreateNestedOneWithoutImagesInput
  }

  export type ImagesUncheckedCreateInput = {
    id?: number
    pid: string
    user_id: number
    training_model_id: number
    pack_id?: number | null
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type ImagesUpdateInput = {
    pid?: StringFieldUpdateOperationsInput | string
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    user?: UsersUpdateOneRequiredWithoutImagesNestedInput
    training_model?: TrainingModelsUpdateOneRequiredWithoutImagesNestedInput
    pack?: PacksUpdateOneWithoutImagesNestedInput
  }

  export type ImagesUncheckedUpdateInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    training_model_id?: IntFieldUpdateOperationsInput | number
    pack_id?: NullableIntFieldUpdateOperationsInput | number | null
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type ImagesCreateManyInput = {
    id?: number
    pid: string
    user_id: number
    training_model_id: number
    pack_id?: number | null
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type ImagesUpdateManyMutationInput = {
    pid?: StringFieldUpdateOperationsInput | string
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type ImagesUncheckedUpdateManyInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    training_model_id?: IntFieldUpdateOperationsInput | number
    pack_id?: NullableIntFieldUpdateOperationsInput | number | null
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type TransactionsCreateInput = {
    pid: string
    credit_amount: number
    model_amount: number
    currency: string
    payment_id: string
    order_id: string
    plan: string
    status: $Enums.Status
    created_at?: Date | string
    updated_at?: Date | string
    user: UsersCreateNestedOneWithoutTransactionsInput
  }

  export type TransactionsUncheckedCreateInput = {
    id?: number
    pid: string
    user_id: number
    credit_amount: number
    model_amount: number
    currency: string
    payment_id: string
    order_id: string
    plan: string
    status: $Enums.Status
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type TransactionsUpdateInput = {
    pid?: StringFieldUpdateOperationsInput | string
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    currency?: StringFieldUpdateOperationsInput | string
    payment_id?: StringFieldUpdateOperationsInput | string
    order_id?: StringFieldUpdateOperationsInput | string
    plan?: StringFieldUpdateOperationsInput | string
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    user?: UsersUpdateOneRequiredWithoutTransactionsNestedInput
  }

  export type TransactionsUncheckedUpdateInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    currency?: StringFieldUpdateOperationsInput | string
    payment_id?: StringFieldUpdateOperationsInput | string
    order_id?: StringFieldUpdateOperationsInput | string
    plan?: StringFieldUpdateOperationsInput | string
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type TransactionsCreateManyInput = {
    id?: number
    pid: string
    user_id: number
    credit_amount: number
    model_amount: number
    currency: string
    payment_id: string
    order_id: string
    plan: string
    status: $Enums.Status
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type TransactionsUpdateManyMutationInput = {
    pid?: StringFieldUpdateOperationsInput | string
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    currency?: StringFieldUpdateOperationsInput | string
    payment_id?: StringFieldUpdateOperationsInput | string
    order_id?: StringFieldUpdateOperationsInput | string
    plan?: StringFieldUpdateOperationsInput | string
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type TransactionsUncheckedUpdateManyInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    currency?: StringFieldUpdateOperationsInput | string
    payment_id?: StringFieldUpdateOperationsInput | string
    order_id?: StringFieldUpdateOperationsInput | string
    plan?: StringFieldUpdateOperationsInput | string
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type PacksCreateInput = {
    pid: string
    name: string
    description: string
    pack_prompts: string
    credits: number
    amount: number
    image_url: string
    created_at?: Date | string
    updated_at?: Date | string
    Images?: ImagesCreateNestedManyWithoutPackInput
  }

  export type PacksUncheckedCreateInput = {
    id?: number
    pid: string
    name: string
    description: string
    pack_prompts: string
    credits: number
    amount: number
    image_url: string
    created_at?: Date | string
    updated_at?: Date | string
    Images?: ImagesUncheckedCreateNestedManyWithoutPackInput
  }

  export type PacksUpdateInput = {
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    description?: StringFieldUpdateOperationsInput | string
    pack_prompts?: StringFieldUpdateOperationsInput | string
    credits?: IntFieldUpdateOperationsInput | number
    amount?: IntFieldUpdateOperationsInput | number
    image_url?: StringFieldUpdateOperationsInput | string
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    Images?: ImagesUpdateManyWithoutPackNestedInput
  }

  export type PacksUncheckedUpdateInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    description?: StringFieldUpdateOperationsInput | string
    pack_prompts?: StringFieldUpdateOperationsInput | string
    credits?: IntFieldUpdateOperationsInput | number
    amount?: IntFieldUpdateOperationsInput | number
    image_url?: StringFieldUpdateOperationsInput | string
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    Images?: ImagesUncheckedUpdateManyWithoutPackNestedInput
  }

  export type PacksCreateManyInput = {
    id?: number
    pid: string
    name: string
    description: string
    pack_prompts: string
    credits: number
    amount: number
    image_url: string
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type PacksUpdateManyMutationInput = {
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    description?: StringFieldUpdateOperationsInput | string
    pack_prompts?: StringFieldUpdateOperationsInput | string
    credits?: IntFieldUpdateOperationsInput | number
    amount?: IntFieldUpdateOperationsInput | number
    image_url?: StringFieldUpdateOperationsInput | string
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type PacksUncheckedUpdateManyInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    description?: StringFieldUpdateOperationsInput | string
    pack_prompts?: StringFieldUpdateOperationsInput | string
    credits?: IntFieldUpdateOperationsInput | number
    amount?: IntFieldUpdateOperationsInput | number
    image_url?: StringFieldUpdateOperationsInput | string
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type IntFilter<$PrismaModel = never> = {
    equals?: number | IntFieldRefInput<$PrismaModel>
    in?: number[] | ListIntFieldRefInput<$PrismaModel>
    notIn?: number[] | ListIntFieldRefInput<$PrismaModel>
    lt?: number | IntFieldRefInput<$PrismaModel>
    lte?: number | IntFieldRefInput<$PrismaModel>
    gt?: number | IntFieldRefInput<$PrismaModel>
    gte?: number | IntFieldRefInput<$PrismaModel>
    not?: NestedIntFilter<$PrismaModel> | number
  }

  export type UuidFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel>
    in?: string[] | ListStringFieldRefInput<$PrismaModel>
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel>
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    mode?: QueryMode
    not?: NestedUuidFilter<$PrismaModel> | string
  }

  export type StringFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel>
    in?: string[] | ListStringFieldRefInput<$PrismaModel>
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel>
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    contains?: string | StringFieldRefInput<$PrismaModel>
    startsWith?: string | StringFieldRefInput<$PrismaModel>
    endsWith?: string | StringFieldRefInput<$PrismaModel>
    mode?: QueryMode
    not?: NestedStringFilter<$PrismaModel> | string
  }

  export type StringNullableFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel> | null
    in?: string[] | ListStringFieldRefInput<$PrismaModel> | null
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel> | null
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    contains?: string | StringFieldRefInput<$PrismaModel>
    startsWith?: string | StringFieldRefInput<$PrismaModel>
    endsWith?: string | StringFieldRefInput<$PrismaModel>
    mode?: QueryMode
    not?: NestedStringNullableFilter<$PrismaModel> | string | null
  }

  export type DateTimeNullableFilter<$PrismaModel = never> = {
    equals?: Date | string | DateTimeFieldRefInput<$PrismaModel> | null
    in?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel> | null
    notIn?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel> | null
    lt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    lte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    not?: NestedDateTimeNullableFilter<$PrismaModel> | Date | string | null
  }

  export type TrainingModelsListRelationFilter = {
    every?: TrainingModelsWhereInput
    some?: TrainingModelsWhereInput
    none?: TrainingModelsWhereInput
  }

  export type UserCreditsListRelationFilter = {
    every?: UserCreditsWhereInput
    some?: UserCreditsWhereInput
    none?: UserCreditsWhereInput
  }

  export type ImagesListRelationFilter = {
    every?: ImagesWhereInput
    some?: ImagesWhereInput
    none?: ImagesWhereInput
  }

  export type TransactionsListRelationFilter = {
    every?: TransactionsWhereInput
    some?: TransactionsWhereInput
    none?: TransactionsWhereInput
  }

  export type SortOrderInput = {
    sort: SortOrder
    nulls?: NullsOrder
  }

  export type TrainingModelsOrderByRelationAggregateInput = {
    _count?: SortOrder
  }

  export type UserCreditsOrderByRelationAggregateInput = {
    _count?: SortOrder
  }

  export type ImagesOrderByRelationAggregateInput = {
    _count?: SortOrder
  }

  export type TransactionsOrderByRelationAggregateInput = {
    _count?: SortOrder
  }

  export type UsersCountOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    email?: SortOrder
    password?: SortOrder
    api_key?: SortOrder
    name?: SortOrder
    reset_token?: SortOrder
    reset_sent_at?: SortOrder
    email_verification_token?: SortOrder
    email_verification_sent_at?: SortOrder
    email_verified_at?: SortOrder
    magicLink_token?: SortOrder
    magicLink_expiration?: SortOrder
  }

  export type UsersAvgOrderByAggregateInput = {
    id?: SortOrder
  }

  export type UsersMaxOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    email?: SortOrder
    password?: SortOrder
    api_key?: SortOrder
    name?: SortOrder
    reset_token?: SortOrder
    reset_sent_at?: SortOrder
    email_verification_token?: SortOrder
    email_verification_sent_at?: SortOrder
    email_verified_at?: SortOrder
    magicLink_token?: SortOrder
    magicLink_expiration?: SortOrder
  }

  export type UsersMinOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    email?: SortOrder
    password?: SortOrder
    api_key?: SortOrder
    name?: SortOrder
    reset_token?: SortOrder
    reset_sent_at?: SortOrder
    email_verification_token?: SortOrder
    email_verification_sent_at?: SortOrder
    email_verified_at?: SortOrder
    magicLink_token?: SortOrder
    magicLink_expiration?: SortOrder
  }

  export type UsersSumOrderByAggregateInput = {
    id?: SortOrder
  }

  export type IntWithAggregatesFilter<$PrismaModel = never> = {
    equals?: number | IntFieldRefInput<$PrismaModel>
    in?: number[] | ListIntFieldRefInput<$PrismaModel>
    notIn?: number[] | ListIntFieldRefInput<$PrismaModel>
    lt?: number | IntFieldRefInput<$PrismaModel>
    lte?: number | IntFieldRefInput<$PrismaModel>
    gt?: number | IntFieldRefInput<$PrismaModel>
    gte?: number | IntFieldRefInput<$PrismaModel>
    not?: NestedIntWithAggregatesFilter<$PrismaModel> | number
    _count?: NestedIntFilter<$PrismaModel>
    _avg?: NestedFloatFilter<$PrismaModel>
    _sum?: NestedIntFilter<$PrismaModel>
    _min?: NestedIntFilter<$PrismaModel>
    _max?: NestedIntFilter<$PrismaModel>
  }

  export type UuidWithAggregatesFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel>
    in?: string[] | ListStringFieldRefInput<$PrismaModel>
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel>
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    mode?: QueryMode
    not?: NestedUuidWithAggregatesFilter<$PrismaModel> | string
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedStringFilter<$PrismaModel>
    _max?: NestedStringFilter<$PrismaModel>
  }

  export type StringWithAggregatesFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel>
    in?: string[] | ListStringFieldRefInput<$PrismaModel>
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel>
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    contains?: string | StringFieldRefInput<$PrismaModel>
    startsWith?: string | StringFieldRefInput<$PrismaModel>
    endsWith?: string | StringFieldRefInput<$PrismaModel>
    mode?: QueryMode
    not?: NestedStringWithAggregatesFilter<$PrismaModel> | string
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedStringFilter<$PrismaModel>
    _max?: NestedStringFilter<$PrismaModel>
  }

  export type StringNullableWithAggregatesFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel> | null
    in?: string[] | ListStringFieldRefInput<$PrismaModel> | null
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel> | null
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    contains?: string | StringFieldRefInput<$PrismaModel>
    startsWith?: string | StringFieldRefInput<$PrismaModel>
    endsWith?: string | StringFieldRefInput<$PrismaModel>
    mode?: QueryMode
    not?: NestedStringNullableWithAggregatesFilter<$PrismaModel> | string | null
    _count?: NestedIntNullableFilter<$PrismaModel>
    _min?: NestedStringNullableFilter<$PrismaModel>
    _max?: NestedStringNullableFilter<$PrismaModel>
  }

  export type DateTimeNullableWithAggregatesFilter<$PrismaModel = never> = {
    equals?: Date | string | DateTimeFieldRefInput<$PrismaModel> | null
    in?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel> | null
    notIn?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel> | null
    lt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    lte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    not?: NestedDateTimeNullableWithAggregatesFilter<$PrismaModel> | Date | string | null
    _count?: NestedIntNullableFilter<$PrismaModel>
    _min?: NestedDateTimeNullableFilter<$PrismaModel>
    _max?: NestedDateTimeNullableFilter<$PrismaModel>
  }

  export type EnumSexFilter<$PrismaModel = never> = {
    equals?: $Enums.Sex | EnumSexFieldRefInput<$PrismaModel>
    in?: $Enums.Sex[] | ListEnumSexFieldRefInput<$PrismaModel>
    notIn?: $Enums.Sex[] | ListEnumSexFieldRefInput<$PrismaModel>
    not?: NestedEnumSexFilter<$PrismaModel> | $Enums.Sex
  }

  export type EnumEthnicityFilter<$PrismaModel = never> = {
    equals?: $Enums.Ethnicity | EnumEthnicityFieldRefInput<$PrismaModel>
    in?: $Enums.Ethnicity[] | ListEnumEthnicityFieldRefInput<$PrismaModel>
    notIn?: $Enums.Ethnicity[] | ListEnumEthnicityFieldRefInput<$PrismaModel>
    not?: NestedEnumEthnicityFilter<$PrismaModel> | $Enums.Ethnicity
  }

  export type EnumBasedOnFilter<$PrismaModel = never> = {
    equals?: $Enums.BasedOn | EnumBasedOnFieldRefInput<$PrismaModel>
    in?: $Enums.BasedOn[] | ListEnumBasedOnFieldRefInput<$PrismaModel>
    notIn?: $Enums.BasedOn[] | ListEnumBasedOnFieldRefInput<$PrismaModel>
    not?: NestedEnumBasedOnFilter<$PrismaModel> | $Enums.BasedOn
  }

  export type EnumEyeColorFilter<$PrismaModel = never> = {
    equals?: $Enums.EyeColor | EnumEyeColorFieldRefInput<$PrismaModel>
    in?: $Enums.EyeColor[] | ListEnumEyeColorFieldRefInput<$PrismaModel>
    notIn?: $Enums.EyeColor[] | ListEnumEyeColorFieldRefInput<$PrismaModel>
    not?: NestedEnumEyeColorFilter<$PrismaModel> | $Enums.EyeColor
  }

  export type BoolFilter<$PrismaModel = never> = {
    equals?: boolean | BooleanFieldRefInput<$PrismaModel>
    not?: NestedBoolFilter<$PrismaModel> | boolean
  }

  export type EnumStatusFilter<$PrismaModel = never> = {
    equals?: $Enums.Status | EnumStatusFieldRefInput<$PrismaModel>
    in?: $Enums.Status[] | ListEnumStatusFieldRefInput<$PrismaModel>
    notIn?: $Enums.Status[] | ListEnumStatusFieldRefInput<$PrismaModel>
    not?: NestedEnumStatusFilter<$PrismaModel> | $Enums.Status
  }
  export type JsonNullableFilter<$PrismaModel = never> = 
    | PatchUndefined<
        Either<Required<JsonNullableFilterBase<$PrismaModel>>, Exclude<keyof Required<JsonNullableFilterBase<$PrismaModel>>, 'path'>>,
        Required<JsonNullableFilterBase<$PrismaModel>>
      >
    | OptionalFlat<Omit<Required<JsonNullableFilterBase<$PrismaModel>>, 'path'>>

  export type JsonNullableFilterBase<$PrismaModel = never> = {
    equals?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | JsonNullValueFilter
    path?: string[]
    mode?: QueryMode | EnumQueryModeFieldRefInput<$PrismaModel>
    string_contains?: string | StringFieldRefInput<$PrismaModel>
    string_starts_with?: string | StringFieldRefInput<$PrismaModel>
    string_ends_with?: string | StringFieldRefInput<$PrismaModel>
    array_starts_with?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | null
    array_ends_with?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | null
    array_contains?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | null
    lt?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    lte?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    gt?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    gte?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    not?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | JsonNullValueFilter
  }

  export type DateTimeFilter<$PrismaModel = never> = {
    equals?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    in?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel>
    notIn?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel>
    lt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    lte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    not?: NestedDateTimeFilter<$PrismaModel> | Date | string
  }

  export type UsersScalarRelationFilter = {
    is?: UsersWhereInput
    isNot?: UsersWhereInput
  }

  export type TrainingModelsCountOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    name?: SortOrder
    age?: SortOrder
    sex?: SortOrder
    ethnicity?: SortOrder
    basedOn?: SortOrder
    eye_color?: SortOrder
    bald?: SortOrder
    steps?: SortOrder
    create_mask?: SortOrder
    is_style?: SortOrder
    trigger_word?: SortOrder
    tensor_path?: SortOrder
    thumbnail?: SortOrder
    training_status?: SortOrder
    fal_output?: SortOrder
    training_images?: SortOrder
    fal_ai_request_id?: SortOrder
    s3_key?: SortOrder
    is_verified?: SortOrder
    deleted_at?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type TrainingModelsAvgOrderByAggregateInput = {
    id?: SortOrder
    user_id?: SortOrder
    age?: SortOrder
    steps?: SortOrder
  }

  export type TrainingModelsMaxOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    name?: SortOrder
    age?: SortOrder
    sex?: SortOrder
    ethnicity?: SortOrder
    basedOn?: SortOrder
    eye_color?: SortOrder
    bald?: SortOrder
    steps?: SortOrder
    create_mask?: SortOrder
    is_style?: SortOrder
    trigger_word?: SortOrder
    tensor_path?: SortOrder
    thumbnail?: SortOrder
    training_status?: SortOrder
    fal_ai_request_id?: SortOrder
    s3_key?: SortOrder
    is_verified?: SortOrder
    deleted_at?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type TrainingModelsMinOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    name?: SortOrder
    age?: SortOrder
    sex?: SortOrder
    ethnicity?: SortOrder
    basedOn?: SortOrder
    eye_color?: SortOrder
    bald?: SortOrder
    steps?: SortOrder
    create_mask?: SortOrder
    is_style?: SortOrder
    trigger_word?: SortOrder
    tensor_path?: SortOrder
    thumbnail?: SortOrder
    training_status?: SortOrder
    fal_ai_request_id?: SortOrder
    s3_key?: SortOrder
    is_verified?: SortOrder
    deleted_at?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type TrainingModelsSumOrderByAggregateInput = {
    id?: SortOrder
    user_id?: SortOrder
    age?: SortOrder
    steps?: SortOrder
  }

  export type EnumSexWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.Sex | EnumSexFieldRefInput<$PrismaModel>
    in?: $Enums.Sex[] | ListEnumSexFieldRefInput<$PrismaModel>
    notIn?: $Enums.Sex[] | ListEnumSexFieldRefInput<$PrismaModel>
    not?: NestedEnumSexWithAggregatesFilter<$PrismaModel> | $Enums.Sex
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumSexFilter<$PrismaModel>
    _max?: NestedEnumSexFilter<$PrismaModel>
  }

  export type EnumEthnicityWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.Ethnicity | EnumEthnicityFieldRefInput<$PrismaModel>
    in?: $Enums.Ethnicity[] | ListEnumEthnicityFieldRefInput<$PrismaModel>
    notIn?: $Enums.Ethnicity[] | ListEnumEthnicityFieldRefInput<$PrismaModel>
    not?: NestedEnumEthnicityWithAggregatesFilter<$PrismaModel> | $Enums.Ethnicity
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumEthnicityFilter<$PrismaModel>
    _max?: NestedEnumEthnicityFilter<$PrismaModel>
  }

  export type EnumBasedOnWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.BasedOn | EnumBasedOnFieldRefInput<$PrismaModel>
    in?: $Enums.BasedOn[] | ListEnumBasedOnFieldRefInput<$PrismaModel>
    notIn?: $Enums.BasedOn[] | ListEnumBasedOnFieldRefInput<$PrismaModel>
    not?: NestedEnumBasedOnWithAggregatesFilter<$PrismaModel> | $Enums.BasedOn
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumBasedOnFilter<$PrismaModel>
    _max?: NestedEnumBasedOnFilter<$PrismaModel>
  }

  export type EnumEyeColorWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.EyeColor | EnumEyeColorFieldRefInput<$PrismaModel>
    in?: $Enums.EyeColor[] | ListEnumEyeColorFieldRefInput<$PrismaModel>
    notIn?: $Enums.EyeColor[] | ListEnumEyeColorFieldRefInput<$PrismaModel>
    not?: NestedEnumEyeColorWithAggregatesFilter<$PrismaModel> | $Enums.EyeColor
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumEyeColorFilter<$PrismaModel>
    _max?: NestedEnumEyeColorFilter<$PrismaModel>
  }

  export type BoolWithAggregatesFilter<$PrismaModel = never> = {
    equals?: boolean | BooleanFieldRefInput<$PrismaModel>
    not?: NestedBoolWithAggregatesFilter<$PrismaModel> | boolean
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedBoolFilter<$PrismaModel>
    _max?: NestedBoolFilter<$PrismaModel>
  }

  export type EnumStatusWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.Status | EnumStatusFieldRefInput<$PrismaModel>
    in?: $Enums.Status[] | ListEnumStatusFieldRefInput<$PrismaModel>
    notIn?: $Enums.Status[] | ListEnumStatusFieldRefInput<$PrismaModel>
    not?: NestedEnumStatusWithAggregatesFilter<$PrismaModel> | $Enums.Status
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumStatusFilter<$PrismaModel>
    _max?: NestedEnumStatusFilter<$PrismaModel>
  }
  export type JsonNullableWithAggregatesFilter<$PrismaModel = never> = 
    | PatchUndefined<
        Either<Required<JsonNullableWithAggregatesFilterBase<$PrismaModel>>, Exclude<keyof Required<JsonNullableWithAggregatesFilterBase<$PrismaModel>>, 'path'>>,
        Required<JsonNullableWithAggregatesFilterBase<$PrismaModel>>
      >
    | OptionalFlat<Omit<Required<JsonNullableWithAggregatesFilterBase<$PrismaModel>>, 'path'>>

  export type JsonNullableWithAggregatesFilterBase<$PrismaModel = never> = {
    equals?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | JsonNullValueFilter
    path?: string[]
    mode?: QueryMode | EnumQueryModeFieldRefInput<$PrismaModel>
    string_contains?: string | StringFieldRefInput<$PrismaModel>
    string_starts_with?: string | StringFieldRefInput<$PrismaModel>
    string_ends_with?: string | StringFieldRefInput<$PrismaModel>
    array_starts_with?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | null
    array_ends_with?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | null
    array_contains?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | null
    lt?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    lte?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    gt?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    gte?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    not?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | JsonNullValueFilter
    _count?: NestedIntNullableFilter<$PrismaModel>
    _min?: NestedJsonNullableFilter<$PrismaModel>
    _max?: NestedJsonNullableFilter<$PrismaModel>
  }

  export type DateTimeWithAggregatesFilter<$PrismaModel = never> = {
    equals?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    in?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel>
    notIn?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel>
    lt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    lte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    not?: NestedDateTimeWithAggregatesFilter<$PrismaModel> | Date | string
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedDateTimeFilter<$PrismaModel>
    _max?: NestedDateTimeFilter<$PrismaModel>
  }

  export type UserCreditsCountOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type UserCreditsAvgOrderByAggregateInput = {
    id?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
  }

  export type UserCreditsMaxOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type UserCreditsMinOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type UserCreditsSumOrderByAggregateInput = {
    id?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
  }

  export type IntNullableFilter<$PrismaModel = never> = {
    equals?: number | IntFieldRefInput<$PrismaModel> | null
    in?: number[] | ListIntFieldRefInput<$PrismaModel> | null
    notIn?: number[] | ListIntFieldRefInput<$PrismaModel> | null
    lt?: number | IntFieldRefInput<$PrismaModel>
    lte?: number | IntFieldRefInput<$PrismaModel>
    gt?: number | IntFieldRefInput<$PrismaModel>
    gte?: number | IntFieldRefInput<$PrismaModel>
    not?: NestedIntNullableFilter<$PrismaModel> | number | null
  }

  export type EnumImageFormatFilter<$PrismaModel = never> = {
    equals?: $Enums.ImageFormat | EnumImageFormatFieldRefInput<$PrismaModel>
    in?: $Enums.ImageFormat[] | ListEnumImageFormatFieldRefInput<$PrismaModel>
    notIn?: $Enums.ImageFormat[] | ListEnumImageFormatFieldRefInput<$PrismaModel>
    not?: NestedEnumImageFormatFilter<$PrismaModel> | $Enums.ImageFormat
  }

  export type EnumImageSizeFilter<$PrismaModel = never> = {
    equals?: $Enums.ImageSize | EnumImageSizeFieldRefInput<$PrismaModel>
    in?: $Enums.ImageSize[] | ListEnumImageSizeFieldRefInput<$PrismaModel>
    notIn?: $Enums.ImageSize[] | ListEnumImageSizeFieldRefInput<$PrismaModel>
    not?: NestedEnumImageSizeFilter<$PrismaModel> | $Enums.ImageSize
  }

  export type TrainingModelsScalarRelationFilter = {
    is?: TrainingModelsWhereInput
    isNot?: TrainingModelsWhereInput
  }

  export type PacksNullableScalarRelationFilter = {
    is?: PacksWhereInput | null
    isNot?: PacksWhereInput | null
  }

  export type ImagesCountOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    training_model_id?: SortOrder
    pack_id?: SortOrder
    user_prompt?: SortOrder
    sys_prompt?: SortOrder
    alt?: SortOrder
    num_inference_steps?: SortOrder
    content_type?: SortOrder
    status?: SortOrder
    image_size?: SortOrder
    fal_ai_request_id?: SortOrder
    width?: SortOrder
    height?: SortOrder
    image_url?: SortOrder
    image_url_s3?: SortOrder
    is_favorite?: SortOrder
    deleted_at?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type ImagesAvgOrderByAggregateInput = {
    id?: SortOrder
    user_id?: SortOrder
    training_model_id?: SortOrder
    pack_id?: SortOrder
    num_inference_steps?: SortOrder
    width?: SortOrder
    height?: SortOrder
  }

  export type ImagesMaxOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    training_model_id?: SortOrder
    pack_id?: SortOrder
    user_prompt?: SortOrder
    sys_prompt?: SortOrder
    alt?: SortOrder
    num_inference_steps?: SortOrder
    content_type?: SortOrder
    status?: SortOrder
    image_size?: SortOrder
    fal_ai_request_id?: SortOrder
    width?: SortOrder
    height?: SortOrder
    image_url?: SortOrder
    image_url_s3?: SortOrder
    is_favorite?: SortOrder
    deleted_at?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type ImagesMinOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    training_model_id?: SortOrder
    pack_id?: SortOrder
    user_prompt?: SortOrder
    sys_prompt?: SortOrder
    alt?: SortOrder
    num_inference_steps?: SortOrder
    content_type?: SortOrder
    status?: SortOrder
    image_size?: SortOrder
    fal_ai_request_id?: SortOrder
    width?: SortOrder
    height?: SortOrder
    image_url?: SortOrder
    image_url_s3?: SortOrder
    is_favorite?: SortOrder
    deleted_at?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type ImagesSumOrderByAggregateInput = {
    id?: SortOrder
    user_id?: SortOrder
    training_model_id?: SortOrder
    pack_id?: SortOrder
    num_inference_steps?: SortOrder
    width?: SortOrder
    height?: SortOrder
  }

  export type IntNullableWithAggregatesFilter<$PrismaModel = never> = {
    equals?: number | IntFieldRefInput<$PrismaModel> | null
    in?: number[] | ListIntFieldRefInput<$PrismaModel> | null
    notIn?: number[] | ListIntFieldRefInput<$PrismaModel> | null
    lt?: number | IntFieldRefInput<$PrismaModel>
    lte?: number | IntFieldRefInput<$PrismaModel>
    gt?: number | IntFieldRefInput<$PrismaModel>
    gte?: number | IntFieldRefInput<$PrismaModel>
    not?: NestedIntNullableWithAggregatesFilter<$PrismaModel> | number | null
    _count?: NestedIntNullableFilter<$PrismaModel>
    _avg?: NestedFloatNullableFilter<$PrismaModel>
    _sum?: NestedIntNullableFilter<$PrismaModel>
    _min?: NestedIntNullableFilter<$PrismaModel>
    _max?: NestedIntNullableFilter<$PrismaModel>
  }

  export type EnumImageFormatWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.ImageFormat | EnumImageFormatFieldRefInput<$PrismaModel>
    in?: $Enums.ImageFormat[] | ListEnumImageFormatFieldRefInput<$PrismaModel>
    notIn?: $Enums.ImageFormat[] | ListEnumImageFormatFieldRefInput<$PrismaModel>
    not?: NestedEnumImageFormatWithAggregatesFilter<$PrismaModel> | $Enums.ImageFormat
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumImageFormatFilter<$PrismaModel>
    _max?: NestedEnumImageFormatFilter<$PrismaModel>
  }

  export type EnumImageSizeWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.ImageSize | EnumImageSizeFieldRefInput<$PrismaModel>
    in?: $Enums.ImageSize[] | ListEnumImageSizeFieldRefInput<$PrismaModel>
    notIn?: $Enums.ImageSize[] | ListEnumImageSizeFieldRefInput<$PrismaModel>
    not?: NestedEnumImageSizeWithAggregatesFilter<$PrismaModel> | $Enums.ImageSize
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumImageSizeFilter<$PrismaModel>
    _max?: NestedEnumImageSizeFilter<$PrismaModel>
  }

  export type TransactionsCountOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
    currency?: SortOrder
    payment_id?: SortOrder
    order_id?: SortOrder
    plan?: SortOrder
    status?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type TransactionsAvgOrderByAggregateInput = {
    id?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
  }

  export type TransactionsMaxOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
    currency?: SortOrder
    payment_id?: SortOrder
    order_id?: SortOrder
    plan?: SortOrder
    status?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type TransactionsMinOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
    currency?: SortOrder
    payment_id?: SortOrder
    order_id?: SortOrder
    plan?: SortOrder
    status?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type TransactionsSumOrderByAggregateInput = {
    id?: SortOrder
    user_id?: SortOrder
    credit_amount?: SortOrder
    model_amount?: SortOrder
  }

  export type PacksCountOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    name?: SortOrder
    description?: SortOrder
    pack_prompts?: SortOrder
    credits?: SortOrder
    amount?: SortOrder
    image_url?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type PacksAvgOrderByAggregateInput = {
    id?: SortOrder
    credits?: SortOrder
    amount?: SortOrder
  }

  export type PacksMaxOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    name?: SortOrder
    description?: SortOrder
    pack_prompts?: SortOrder
    credits?: SortOrder
    amount?: SortOrder
    image_url?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type PacksMinOrderByAggregateInput = {
    id?: SortOrder
    pid?: SortOrder
    name?: SortOrder
    description?: SortOrder
    pack_prompts?: SortOrder
    credits?: SortOrder
    amount?: SortOrder
    image_url?: SortOrder
    created_at?: SortOrder
    updated_at?: SortOrder
  }

  export type PacksSumOrderByAggregateInput = {
    id?: SortOrder
    credits?: SortOrder
    amount?: SortOrder
  }

  export type TrainingModelsCreateNestedManyWithoutUserInput = {
    create?: XOR<TrainingModelsCreateWithoutUserInput, TrainingModelsUncheckedCreateWithoutUserInput> | TrainingModelsCreateWithoutUserInput[] | TrainingModelsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: TrainingModelsCreateOrConnectWithoutUserInput | TrainingModelsCreateOrConnectWithoutUserInput[]
    createMany?: TrainingModelsCreateManyUserInputEnvelope
    connect?: TrainingModelsWhereUniqueInput | TrainingModelsWhereUniqueInput[]
  }

  export type UserCreditsCreateNestedManyWithoutUserInput = {
    create?: XOR<UserCreditsCreateWithoutUserInput, UserCreditsUncheckedCreateWithoutUserInput> | UserCreditsCreateWithoutUserInput[] | UserCreditsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: UserCreditsCreateOrConnectWithoutUserInput | UserCreditsCreateOrConnectWithoutUserInput[]
    createMany?: UserCreditsCreateManyUserInputEnvelope
    connect?: UserCreditsWhereUniqueInput | UserCreditsWhereUniqueInput[]
  }

  export type ImagesCreateNestedManyWithoutUserInput = {
    create?: XOR<ImagesCreateWithoutUserInput, ImagesUncheckedCreateWithoutUserInput> | ImagesCreateWithoutUserInput[] | ImagesUncheckedCreateWithoutUserInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutUserInput | ImagesCreateOrConnectWithoutUserInput[]
    createMany?: ImagesCreateManyUserInputEnvelope
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
  }

  export type TransactionsCreateNestedManyWithoutUserInput = {
    create?: XOR<TransactionsCreateWithoutUserInput, TransactionsUncheckedCreateWithoutUserInput> | TransactionsCreateWithoutUserInput[] | TransactionsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: TransactionsCreateOrConnectWithoutUserInput | TransactionsCreateOrConnectWithoutUserInput[]
    createMany?: TransactionsCreateManyUserInputEnvelope
    connect?: TransactionsWhereUniqueInput | TransactionsWhereUniqueInput[]
  }

  export type TrainingModelsUncheckedCreateNestedManyWithoutUserInput = {
    create?: XOR<TrainingModelsCreateWithoutUserInput, TrainingModelsUncheckedCreateWithoutUserInput> | TrainingModelsCreateWithoutUserInput[] | TrainingModelsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: TrainingModelsCreateOrConnectWithoutUserInput | TrainingModelsCreateOrConnectWithoutUserInput[]
    createMany?: TrainingModelsCreateManyUserInputEnvelope
    connect?: TrainingModelsWhereUniqueInput | TrainingModelsWhereUniqueInput[]
  }

  export type UserCreditsUncheckedCreateNestedManyWithoutUserInput = {
    create?: XOR<UserCreditsCreateWithoutUserInput, UserCreditsUncheckedCreateWithoutUserInput> | UserCreditsCreateWithoutUserInput[] | UserCreditsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: UserCreditsCreateOrConnectWithoutUserInput | UserCreditsCreateOrConnectWithoutUserInput[]
    createMany?: UserCreditsCreateManyUserInputEnvelope
    connect?: UserCreditsWhereUniqueInput | UserCreditsWhereUniqueInput[]
  }

  export type ImagesUncheckedCreateNestedManyWithoutUserInput = {
    create?: XOR<ImagesCreateWithoutUserInput, ImagesUncheckedCreateWithoutUserInput> | ImagesCreateWithoutUserInput[] | ImagesUncheckedCreateWithoutUserInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutUserInput | ImagesCreateOrConnectWithoutUserInput[]
    createMany?: ImagesCreateManyUserInputEnvelope
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
  }

  export type TransactionsUncheckedCreateNestedManyWithoutUserInput = {
    create?: XOR<TransactionsCreateWithoutUserInput, TransactionsUncheckedCreateWithoutUserInput> | TransactionsCreateWithoutUserInput[] | TransactionsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: TransactionsCreateOrConnectWithoutUserInput | TransactionsCreateOrConnectWithoutUserInput[]
    createMany?: TransactionsCreateManyUserInputEnvelope
    connect?: TransactionsWhereUniqueInput | TransactionsWhereUniqueInput[]
  }

  export type StringFieldUpdateOperationsInput = {
    set?: string
  }

  export type NullableStringFieldUpdateOperationsInput = {
    set?: string | null
  }

  export type NullableDateTimeFieldUpdateOperationsInput = {
    set?: Date | string | null
  }

  export type TrainingModelsUpdateManyWithoutUserNestedInput = {
    create?: XOR<TrainingModelsCreateWithoutUserInput, TrainingModelsUncheckedCreateWithoutUserInput> | TrainingModelsCreateWithoutUserInput[] | TrainingModelsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: TrainingModelsCreateOrConnectWithoutUserInput | TrainingModelsCreateOrConnectWithoutUserInput[]
    upsert?: TrainingModelsUpsertWithWhereUniqueWithoutUserInput | TrainingModelsUpsertWithWhereUniqueWithoutUserInput[]
    createMany?: TrainingModelsCreateManyUserInputEnvelope
    set?: TrainingModelsWhereUniqueInput | TrainingModelsWhereUniqueInput[]
    disconnect?: TrainingModelsWhereUniqueInput | TrainingModelsWhereUniqueInput[]
    delete?: TrainingModelsWhereUniqueInput | TrainingModelsWhereUniqueInput[]
    connect?: TrainingModelsWhereUniqueInput | TrainingModelsWhereUniqueInput[]
    update?: TrainingModelsUpdateWithWhereUniqueWithoutUserInput | TrainingModelsUpdateWithWhereUniqueWithoutUserInput[]
    updateMany?: TrainingModelsUpdateManyWithWhereWithoutUserInput | TrainingModelsUpdateManyWithWhereWithoutUserInput[]
    deleteMany?: TrainingModelsScalarWhereInput | TrainingModelsScalarWhereInput[]
  }

  export type UserCreditsUpdateManyWithoutUserNestedInput = {
    create?: XOR<UserCreditsCreateWithoutUserInput, UserCreditsUncheckedCreateWithoutUserInput> | UserCreditsCreateWithoutUserInput[] | UserCreditsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: UserCreditsCreateOrConnectWithoutUserInput | UserCreditsCreateOrConnectWithoutUserInput[]
    upsert?: UserCreditsUpsertWithWhereUniqueWithoutUserInput | UserCreditsUpsertWithWhereUniqueWithoutUserInput[]
    createMany?: UserCreditsCreateManyUserInputEnvelope
    set?: UserCreditsWhereUniqueInput | UserCreditsWhereUniqueInput[]
    disconnect?: UserCreditsWhereUniqueInput | UserCreditsWhereUniqueInput[]
    delete?: UserCreditsWhereUniqueInput | UserCreditsWhereUniqueInput[]
    connect?: UserCreditsWhereUniqueInput | UserCreditsWhereUniqueInput[]
    update?: UserCreditsUpdateWithWhereUniqueWithoutUserInput | UserCreditsUpdateWithWhereUniqueWithoutUserInput[]
    updateMany?: UserCreditsUpdateManyWithWhereWithoutUserInput | UserCreditsUpdateManyWithWhereWithoutUserInput[]
    deleteMany?: UserCreditsScalarWhereInput | UserCreditsScalarWhereInput[]
  }

  export type ImagesUpdateManyWithoutUserNestedInput = {
    create?: XOR<ImagesCreateWithoutUserInput, ImagesUncheckedCreateWithoutUserInput> | ImagesCreateWithoutUserInput[] | ImagesUncheckedCreateWithoutUserInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutUserInput | ImagesCreateOrConnectWithoutUserInput[]
    upsert?: ImagesUpsertWithWhereUniqueWithoutUserInput | ImagesUpsertWithWhereUniqueWithoutUserInput[]
    createMany?: ImagesCreateManyUserInputEnvelope
    set?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    disconnect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    delete?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    update?: ImagesUpdateWithWhereUniqueWithoutUserInput | ImagesUpdateWithWhereUniqueWithoutUserInput[]
    updateMany?: ImagesUpdateManyWithWhereWithoutUserInput | ImagesUpdateManyWithWhereWithoutUserInput[]
    deleteMany?: ImagesScalarWhereInput | ImagesScalarWhereInput[]
  }

  export type TransactionsUpdateManyWithoutUserNestedInput = {
    create?: XOR<TransactionsCreateWithoutUserInput, TransactionsUncheckedCreateWithoutUserInput> | TransactionsCreateWithoutUserInput[] | TransactionsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: TransactionsCreateOrConnectWithoutUserInput | TransactionsCreateOrConnectWithoutUserInput[]
    upsert?: TransactionsUpsertWithWhereUniqueWithoutUserInput | TransactionsUpsertWithWhereUniqueWithoutUserInput[]
    createMany?: TransactionsCreateManyUserInputEnvelope
    set?: TransactionsWhereUniqueInput | TransactionsWhereUniqueInput[]
    disconnect?: TransactionsWhereUniqueInput | TransactionsWhereUniqueInput[]
    delete?: TransactionsWhereUniqueInput | TransactionsWhereUniqueInput[]
    connect?: TransactionsWhereUniqueInput | TransactionsWhereUniqueInput[]
    update?: TransactionsUpdateWithWhereUniqueWithoutUserInput | TransactionsUpdateWithWhereUniqueWithoutUserInput[]
    updateMany?: TransactionsUpdateManyWithWhereWithoutUserInput | TransactionsUpdateManyWithWhereWithoutUserInput[]
    deleteMany?: TransactionsScalarWhereInput | TransactionsScalarWhereInput[]
  }

  export type IntFieldUpdateOperationsInput = {
    set?: number
    increment?: number
    decrement?: number
    multiply?: number
    divide?: number
  }

  export type TrainingModelsUncheckedUpdateManyWithoutUserNestedInput = {
    create?: XOR<TrainingModelsCreateWithoutUserInput, TrainingModelsUncheckedCreateWithoutUserInput> | TrainingModelsCreateWithoutUserInput[] | TrainingModelsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: TrainingModelsCreateOrConnectWithoutUserInput | TrainingModelsCreateOrConnectWithoutUserInput[]
    upsert?: TrainingModelsUpsertWithWhereUniqueWithoutUserInput | TrainingModelsUpsertWithWhereUniqueWithoutUserInput[]
    createMany?: TrainingModelsCreateManyUserInputEnvelope
    set?: TrainingModelsWhereUniqueInput | TrainingModelsWhereUniqueInput[]
    disconnect?: TrainingModelsWhereUniqueInput | TrainingModelsWhereUniqueInput[]
    delete?: TrainingModelsWhereUniqueInput | TrainingModelsWhereUniqueInput[]
    connect?: TrainingModelsWhereUniqueInput | TrainingModelsWhereUniqueInput[]
    update?: TrainingModelsUpdateWithWhereUniqueWithoutUserInput | TrainingModelsUpdateWithWhereUniqueWithoutUserInput[]
    updateMany?: TrainingModelsUpdateManyWithWhereWithoutUserInput | TrainingModelsUpdateManyWithWhereWithoutUserInput[]
    deleteMany?: TrainingModelsScalarWhereInput | TrainingModelsScalarWhereInput[]
  }

  export type UserCreditsUncheckedUpdateManyWithoutUserNestedInput = {
    create?: XOR<UserCreditsCreateWithoutUserInput, UserCreditsUncheckedCreateWithoutUserInput> | UserCreditsCreateWithoutUserInput[] | UserCreditsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: UserCreditsCreateOrConnectWithoutUserInput | UserCreditsCreateOrConnectWithoutUserInput[]
    upsert?: UserCreditsUpsertWithWhereUniqueWithoutUserInput | UserCreditsUpsertWithWhereUniqueWithoutUserInput[]
    createMany?: UserCreditsCreateManyUserInputEnvelope
    set?: UserCreditsWhereUniqueInput | UserCreditsWhereUniqueInput[]
    disconnect?: UserCreditsWhereUniqueInput | UserCreditsWhereUniqueInput[]
    delete?: UserCreditsWhereUniqueInput | UserCreditsWhereUniqueInput[]
    connect?: UserCreditsWhereUniqueInput | UserCreditsWhereUniqueInput[]
    update?: UserCreditsUpdateWithWhereUniqueWithoutUserInput | UserCreditsUpdateWithWhereUniqueWithoutUserInput[]
    updateMany?: UserCreditsUpdateManyWithWhereWithoutUserInput | UserCreditsUpdateManyWithWhereWithoutUserInput[]
    deleteMany?: UserCreditsScalarWhereInput | UserCreditsScalarWhereInput[]
  }

  export type ImagesUncheckedUpdateManyWithoutUserNestedInput = {
    create?: XOR<ImagesCreateWithoutUserInput, ImagesUncheckedCreateWithoutUserInput> | ImagesCreateWithoutUserInput[] | ImagesUncheckedCreateWithoutUserInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutUserInput | ImagesCreateOrConnectWithoutUserInput[]
    upsert?: ImagesUpsertWithWhereUniqueWithoutUserInput | ImagesUpsertWithWhereUniqueWithoutUserInput[]
    createMany?: ImagesCreateManyUserInputEnvelope
    set?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    disconnect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    delete?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    update?: ImagesUpdateWithWhereUniqueWithoutUserInput | ImagesUpdateWithWhereUniqueWithoutUserInput[]
    updateMany?: ImagesUpdateManyWithWhereWithoutUserInput | ImagesUpdateManyWithWhereWithoutUserInput[]
    deleteMany?: ImagesScalarWhereInput | ImagesScalarWhereInput[]
  }

  export type TransactionsUncheckedUpdateManyWithoutUserNestedInput = {
    create?: XOR<TransactionsCreateWithoutUserInput, TransactionsUncheckedCreateWithoutUserInput> | TransactionsCreateWithoutUserInput[] | TransactionsUncheckedCreateWithoutUserInput[]
    connectOrCreate?: TransactionsCreateOrConnectWithoutUserInput | TransactionsCreateOrConnectWithoutUserInput[]
    upsert?: TransactionsUpsertWithWhereUniqueWithoutUserInput | TransactionsUpsertWithWhereUniqueWithoutUserInput[]
    createMany?: TransactionsCreateManyUserInputEnvelope
    set?: TransactionsWhereUniqueInput | TransactionsWhereUniqueInput[]
    disconnect?: TransactionsWhereUniqueInput | TransactionsWhereUniqueInput[]
    delete?: TransactionsWhereUniqueInput | TransactionsWhereUniqueInput[]
    connect?: TransactionsWhereUniqueInput | TransactionsWhereUniqueInput[]
    update?: TransactionsUpdateWithWhereUniqueWithoutUserInput | TransactionsUpdateWithWhereUniqueWithoutUserInput[]
    updateMany?: TransactionsUpdateManyWithWhereWithoutUserInput | TransactionsUpdateManyWithWhereWithoutUserInput[]
    deleteMany?: TransactionsScalarWhereInput | TransactionsScalarWhereInput[]
  }

  export type UsersCreateNestedOneWithoutTrainingModelsInput = {
    create?: XOR<UsersCreateWithoutTrainingModelsInput, UsersUncheckedCreateWithoutTrainingModelsInput>
    connectOrCreate?: UsersCreateOrConnectWithoutTrainingModelsInput
    connect?: UsersWhereUniqueInput
  }

  export type ImagesCreateNestedManyWithoutTraining_modelInput = {
    create?: XOR<ImagesCreateWithoutTraining_modelInput, ImagesUncheckedCreateWithoutTraining_modelInput> | ImagesCreateWithoutTraining_modelInput[] | ImagesUncheckedCreateWithoutTraining_modelInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutTraining_modelInput | ImagesCreateOrConnectWithoutTraining_modelInput[]
    createMany?: ImagesCreateManyTraining_modelInputEnvelope
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
  }

  export type ImagesUncheckedCreateNestedManyWithoutTraining_modelInput = {
    create?: XOR<ImagesCreateWithoutTraining_modelInput, ImagesUncheckedCreateWithoutTraining_modelInput> | ImagesCreateWithoutTraining_modelInput[] | ImagesUncheckedCreateWithoutTraining_modelInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutTraining_modelInput | ImagesCreateOrConnectWithoutTraining_modelInput[]
    createMany?: ImagesCreateManyTraining_modelInputEnvelope
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
  }

  export type EnumSexFieldUpdateOperationsInput = {
    set?: $Enums.Sex
  }

  export type EnumEthnicityFieldUpdateOperationsInput = {
    set?: $Enums.Ethnicity
  }

  export type EnumBasedOnFieldUpdateOperationsInput = {
    set?: $Enums.BasedOn
  }

  export type EnumEyeColorFieldUpdateOperationsInput = {
    set?: $Enums.EyeColor
  }

  export type BoolFieldUpdateOperationsInput = {
    set?: boolean
  }

  export type EnumStatusFieldUpdateOperationsInput = {
    set?: $Enums.Status
  }

  export type DateTimeFieldUpdateOperationsInput = {
    set?: Date | string
  }

  export type UsersUpdateOneRequiredWithoutTrainingModelsNestedInput = {
    create?: XOR<UsersCreateWithoutTrainingModelsInput, UsersUncheckedCreateWithoutTrainingModelsInput>
    connectOrCreate?: UsersCreateOrConnectWithoutTrainingModelsInput
    upsert?: UsersUpsertWithoutTrainingModelsInput
    connect?: UsersWhereUniqueInput
    update?: XOR<XOR<UsersUpdateToOneWithWhereWithoutTrainingModelsInput, UsersUpdateWithoutTrainingModelsInput>, UsersUncheckedUpdateWithoutTrainingModelsInput>
  }

  export type ImagesUpdateManyWithoutTraining_modelNestedInput = {
    create?: XOR<ImagesCreateWithoutTraining_modelInput, ImagesUncheckedCreateWithoutTraining_modelInput> | ImagesCreateWithoutTraining_modelInput[] | ImagesUncheckedCreateWithoutTraining_modelInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutTraining_modelInput | ImagesCreateOrConnectWithoutTraining_modelInput[]
    upsert?: ImagesUpsertWithWhereUniqueWithoutTraining_modelInput | ImagesUpsertWithWhereUniqueWithoutTraining_modelInput[]
    createMany?: ImagesCreateManyTraining_modelInputEnvelope
    set?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    disconnect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    delete?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    update?: ImagesUpdateWithWhereUniqueWithoutTraining_modelInput | ImagesUpdateWithWhereUniqueWithoutTraining_modelInput[]
    updateMany?: ImagesUpdateManyWithWhereWithoutTraining_modelInput | ImagesUpdateManyWithWhereWithoutTraining_modelInput[]
    deleteMany?: ImagesScalarWhereInput | ImagesScalarWhereInput[]
  }

  export type ImagesUncheckedUpdateManyWithoutTraining_modelNestedInput = {
    create?: XOR<ImagesCreateWithoutTraining_modelInput, ImagesUncheckedCreateWithoutTraining_modelInput> | ImagesCreateWithoutTraining_modelInput[] | ImagesUncheckedCreateWithoutTraining_modelInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutTraining_modelInput | ImagesCreateOrConnectWithoutTraining_modelInput[]
    upsert?: ImagesUpsertWithWhereUniqueWithoutTraining_modelInput | ImagesUpsertWithWhereUniqueWithoutTraining_modelInput[]
    createMany?: ImagesCreateManyTraining_modelInputEnvelope
    set?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    disconnect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    delete?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    update?: ImagesUpdateWithWhereUniqueWithoutTraining_modelInput | ImagesUpdateWithWhereUniqueWithoutTraining_modelInput[]
    updateMany?: ImagesUpdateManyWithWhereWithoutTraining_modelInput | ImagesUpdateManyWithWhereWithoutTraining_modelInput[]
    deleteMany?: ImagesScalarWhereInput | ImagesScalarWhereInput[]
  }

  export type UsersCreateNestedOneWithoutUserCreditsInput = {
    create?: XOR<UsersCreateWithoutUserCreditsInput, UsersUncheckedCreateWithoutUserCreditsInput>
    connectOrCreate?: UsersCreateOrConnectWithoutUserCreditsInput
    connect?: UsersWhereUniqueInput
  }

  export type UsersUpdateOneRequiredWithoutUserCreditsNestedInput = {
    create?: XOR<UsersCreateWithoutUserCreditsInput, UsersUncheckedCreateWithoutUserCreditsInput>
    connectOrCreate?: UsersCreateOrConnectWithoutUserCreditsInput
    upsert?: UsersUpsertWithoutUserCreditsInput
    connect?: UsersWhereUniqueInput
    update?: XOR<XOR<UsersUpdateToOneWithWhereWithoutUserCreditsInput, UsersUpdateWithoutUserCreditsInput>, UsersUncheckedUpdateWithoutUserCreditsInput>
  }

  export type UsersCreateNestedOneWithoutImagesInput = {
    create?: XOR<UsersCreateWithoutImagesInput, UsersUncheckedCreateWithoutImagesInput>
    connectOrCreate?: UsersCreateOrConnectWithoutImagesInput
    connect?: UsersWhereUniqueInput
  }

  export type TrainingModelsCreateNestedOneWithoutImagesInput = {
    create?: XOR<TrainingModelsCreateWithoutImagesInput, TrainingModelsUncheckedCreateWithoutImagesInput>
    connectOrCreate?: TrainingModelsCreateOrConnectWithoutImagesInput
    connect?: TrainingModelsWhereUniqueInput
  }

  export type PacksCreateNestedOneWithoutImagesInput = {
    create?: XOR<PacksCreateWithoutImagesInput, PacksUncheckedCreateWithoutImagesInput>
    connectOrCreate?: PacksCreateOrConnectWithoutImagesInput
    connect?: PacksWhereUniqueInput
  }

  export type EnumImageFormatFieldUpdateOperationsInput = {
    set?: $Enums.ImageFormat
  }

  export type EnumImageSizeFieldUpdateOperationsInput = {
    set?: $Enums.ImageSize
  }

  export type NullableIntFieldUpdateOperationsInput = {
    set?: number | null
    increment?: number
    decrement?: number
    multiply?: number
    divide?: number
  }

  export type UsersUpdateOneRequiredWithoutImagesNestedInput = {
    create?: XOR<UsersCreateWithoutImagesInput, UsersUncheckedCreateWithoutImagesInput>
    connectOrCreate?: UsersCreateOrConnectWithoutImagesInput
    upsert?: UsersUpsertWithoutImagesInput
    connect?: UsersWhereUniqueInput
    update?: XOR<XOR<UsersUpdateToOneWithWhereWithoutImagesInput, UsersUpdateWithoutImagesInput>, UsersUncheckedUpdateWithoutImagesInput>
  }

  export type TrainingModelsUpdateOneRequiredWithoutImagesNestedInput = {
    create?: XOR<TrainingModelsCreateWithoutImagesInput, TrainingModelsUncheckedCreateWithoutImagesInput>
    connectOrCreate?: TrainingModelsCreateOrConnectWithoutImagesInput
    upsert?: TrainingModelsUpsertWithoutImagesInput
    connect?: TrainingModelsWhereUniqueInput
    update?: XOR<XOR<TrainingModelsUpdateToOneWithWhereWithoutImagesInput, TrainingModelsUpdateWithoutImagesInput>, TrainingModelsUncheckedUpdateWithoutImagesInput>
  }

  export type PacksUpdateOneWithoutImagesNestedInput = {
    create?: XOR<PacksCreateWithoutImagesInput, PacksUncheckedCreateWithoutImagesInput>
    connectOrCreate?: PacksCreateOrConnectWithoutImagesInput
    upsert?: PacksUpsertWithoutImagesInput
    disconnect?: PacksWhereInput | boolean
    delete?: PacksWhereInput | boolean
    connect?: PacksWhereUniqueInput
    update?: XOR<XOR<PacksUpdateToOneWithWhereWithoutImagesInput, PacksUpdateWithoutImagesInput>, PacksUncheckedUpdateWithoutImagesInput>
  }

  export type UsersCreateNestedOneWithoutTransactionsInput = {
    create?: XOR<UsersCreateWithoutTransactionsInput, UsersUncheckedCreateWithoutTransactionsInput>
    connectOrCreate?: UsersCreateOrConnectWithoutTransactionsInput
    connect?: UsersWhereUniqueInput
  }

  export type UsersUpdateOneRequiredWithoutTransactionsNestedInput = {
    create?: XOR<UsersCreateWithoutTransactionsInput, UsersUncheckedCreateWithoutTransactionsInput>
    connectOrCreate?: UsersCreateOrConnectWithoutTransactionsInput
    upsert?: UsersUpsertWithoutTransactionsInput
    connect?: UsersWhereUniqueInput
    update?: XOR<XOR<UsersUpdateToOneWithWhereWithoutTransactionsInput, UsersUpdateWithoutTransactionsInput>, UsersUncheckedUpdateWithoutTransactionsInput>
  }

  export type ImagesCreateNestedManyWithoutPackInput = {
    create?: XOR<ImagesCreateWithoutPackInput, ImagesUncheckedCreateWithoutPackInput> | ImagesCreateWithoutPackInput[] | ImagesUncheckedCreateWithoutPackInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutPackInput | ImagesCreateOrConnectWithoutPackInput[]
    createMany?: ImagesCreateManyPackInputEnvelope
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
  }

  export type ImagesUncheckedCreateNestedManyWithoutPackInput = {
    create?: XOR<ImagesCreateWithoutPackInput, ImagesUncheckedCreateWithoutPackInput> | ImagesCreateWithoutPackInput[] | ImagesUncheckedCreateWithoutPackInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutPackInput | ImagesCreateOrConnectWithoutPackInput[]
    createMany?: ImagesCreateManyPackInputEnvelope
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
  }

  export type ImagesUpdateManyWithoutPackNestedInput = {
    create?: XOR<ImagesCreateWithoutPackInput, ImagesUncheckedCreateWithoutPackInput> | ImagesCreateWithoutPackInput[] | ImagesUncheckedCreateWithoutPackInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutPackInput | ImagesCreateOrConnectWithoutPackInput[]
    upsert?: ImagesUpsertWithWhereUniqueWithoutPackInput | ImagesUpsertWithWhereUniqueWithoutPackInput[]
    createMany?: ImagesCreateManyPackInputEnvelope
    set?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    disconnect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    delete?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    update?: ImagesUpdateWithWhereUniqueWithoutPackInput | ImagesUpdateWithWhereUniqueWithoutPackInput[]
    updateMany?: ImagesUpdateManyWithWhereWithoutPackInput | ImagesUpdateManyWithWhereWithoutPackInput[]
    deleteMany?: ImagesScalarWhereInput | ImagesScalarWhereInput[]
  }

  export type ImagesUncheckedUpdateManyWithoutPackNestedInput = {
    create?: XOR<ImagesCreateWithoutPackInput, ImagesUncheckedCreateWithoutPackInput> | ImagesCreateWithoutPackInput[] | ImagesUncheckedCreateWithoutPackInput[]
    connectOrCreate?: ImagesCreateOrConnectWithoutPackInput | ImagesCreateOrConnectWithoutPackInput[]
    upsert?: ImagesUpsertWithWhereUniqueWithoutPackInput | ImagesUpsertWithWhereUniqueWithoutPackInput[]
    createMany?: ImagesCreateManyPackInputEnvelope
    set?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    disconnect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    delete?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    connect?: ImagesWhereUniqueInput | ImagesWhereUniqueInput[]
    update?: ImagesUpdateWithWhereUniqueWithoutPackInput | ImagesUpdateWithWhereUniqueWithoutPackInput[]
    updateMany?: ImagesUpdateManyWithWhereWithoutPackInput | ImagesUpdateManyWithWhereWithoutPackInput[]
    deleteMany?: ImagesScalarWhereInput | ImagesScalarWhereInput[]
  }

  export type NestedIntFilter<$PrismaModel = never> = {
    equals?: number | IntFieldRefInput<$PrismaModel>
    in?: number[] | ListIntFieldRefInput<$PrismaModel>
    notIn?: number[] | ListIntFieldRefInput<$PrismaModel>
    lt?: number | IntFieldRefInput<$PrismaModel>
    lte?: number | IntFieldRefInput<$PrismaModel>
    gt?: number | IntFieldRefInput<$PrismaModel>
    gte?: number | IntFieldRefInput<$PrismaModel>
    not?: NestedIntFilter<$PrismaModel> | number
  }

  export type NestedUuidFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel>
    in?: string[] | ListStringFieldRefInput<$PrismaModel>
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel>
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    not?: NestedUuidFilter<$PrismaModel> | string
  }

  export type NestedStringFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel>
    in?: string[] | ListStringFieldRefInput<$PrismaModel>
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel>
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    contains?: string | StringFieldRefInput<$PrismaModel>
    startsWith?: string | StringFieldRefInput<$PrismaModel>
    endsWith?: string | StringFieldRefInput<$PrismaModel>
    not?: NestedStringFilter<$PrismaModel> | string
  }

  export type NestedStringNullableFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel> | null
    in?: string[] | ListStringFieldRefInput<$PrismaModel> | null
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel> | null
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    contains?: string | StringFieldRefInput<$PrismaModel>
    startsWith?: string | StringFieldRefInput<$PrismaModel>
    endsWith?: string | StringFieldRefInput<$PrismaModel>
    not?: NestedStringNullableFilter<$PrismaModel> | string | null
  }

  export type NestedDateTimeNullableFilter<$PrismaModel = never> = {
    equals?: Date | string | DateTimeFieldRefInput<$PrismaModel> | null
    in?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel> | null
    notIn?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel> | null
    lt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    lte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    not?: NestedDateTimeNullableFilter<$PrismaModel> | Date | string | null
  }

  export type NestedIntWithAggregatesFilter<$PrismaModel = never> = {
    equals?: number | IntFieldRefInput<$PrismaModel>
    in?: number[] | ListIntFieldRefInput<$PrismaModel>
    notIn?: number[] | ListIntFieldRefInput<$PrismaModel>
    lt?: number | IntFieldRefInput<$PrismaModel>
    lte?: number | IntFieldRefInput<$PrismaModel>
    gt?: number | IntFieldRefInput<$PrismaModel>
    gte?: number | IntFieldRefInput<$PrismaModel>
    not?: NestedIntWithAggregatesFilter<$PrismaModel> | number
    _count?: NestedIntFilter<$PrismaModel>
    _avg?: NestedFloatFilter<$PrismaModel>
    _sum?: NestedIntFilter<$PrismaModel>
    _min?: NestedIntFilter<$PrismaModel>
    _max?: NestedIntFilter<$PrismaModel>
  }

  export type NestedFloatFilter<$PrismaModel = never> = {
    equals?: number | FloatFieldRefInput<$PrismaModel>
    in?: number[] | ListFloatFieldRefInput<$PrismaModel>
    notIn?: number[] | ListFloatFieldRefInput<$PrismaModel>
    lt?: number | FloatFieldRefInput<$PrismaModel>
    lte?: number | FloatFieldRefInput<$PrismaModel>
    gt?: number | FloatFieldRefInput<$PrismaModel>
    gte?: number | FloatFieldRefInput<$PrismaModel>
    not?: NestedFloatFilter<$PrismaModel> | number
  }

  export type NestedUuidWithAggregatesFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel>
    in?: string[] | ListStringFieldRefInput<$PrismaModel>
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel>
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    not?: NestedUuidWithAggregatesFilter<$PrismaModel> | string
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedStringFilter<$PrismaModel>
    _max?: NestedStringFilter<$PrismaModel>
  }

  export type NestedStringWithAggregatesFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel>
    in?: string[] | ListStringFieldRefInput<$PrismaModel>
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel>
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    contains?: string | StringFieldRefInput<$PrismaModel>
    startsWith?: string | StringFieldRefInput<$PrismaModel>
    endsWith?: string | StringFieldRefInput<$PrismaModel>
    not?: NestedStringWithAggregatesFilter<$PrismaModel> | string
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedStringFilter<$PrismaModel>
    _max?: NestedStringFilter<$PrismaModel>
  }

  export type NestedStringNullableWithAggregatesFilter<$PrismaModel = never> = {
    equals?: string | StringFieldRefInput<$PrismaModel> | null
    in?: string[] | ListStringFieldRefInput<$PrismaModel> | null
    notIn?: string[] | ListStringFieldRefInput<$PrismaModel> | null
    lt?: string | StringFieldRefInput<$PrismaModel>
    lte?: string | StringFieldRefInput<$PrismaModel>
    gt?: string | StringFieldRefInput<$PrismaModel>
    gte?: string | StringFieldRefInput<$PrismaModel>
    contains?: string | StringFieldRefInput<$PrismaModel>
    startsWith?: string | StringFieldRefInput<$PrismaModel>
    endsWith?: string | StringFieldRefInput<$PrismaModel>
    not?: NestedStringNullableWithAggregatesFilter<$PrismaModel> | string | null
    _count?: NestedIntNullableFilter<$PrismaModel>
    _min?: NestedStringNullableFilter<$PrismaModel>
    _max?: NestedStringNullableFilter<$PrismaModel>
  }

  export type NestedIntNullableFilter<$PrismaModel = never> = {
    equals?: number | IntFieldRefInput<$PrismaModel> | null
    in?: number[] | ListIntFieldRefInput<$PrismaModel> | null
    notIn?: number[] | ListIntFieldRefInput<$PrismaModel> | null
    lt?: number | IntFieldRefInput<$PrismaModel>
    lte?: number | IntFieldRefInput<$PrismaModel>
    gt?: number | IntFieldRefInput<$PrismaModel>
    gte?: number | IntFieldRefInput<$PrismaModel>
    not?: NestedIntNullableFilter<$PrismaModel> | number | null
  }

  export type NestedDateTimeNullableWithAggregatesFilter<$PrismaModel = never> = {
    equals?: Date | string | DateTimeFieldRefInput<$PrismaModel> | null
    in?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel> | null
    notIn?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel> | null
    lt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    lte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    not?: NestedDateTimeNullableWithAggregatesFilter<$PrismaModel> | Date | string | null
    _count?: NestedIntNullableFilter<$PrismaModel>
    _min?: NestedDateTimeNullableFilter<$PrismaModel>
    _max?: NestedDateTimeNullableFilter<$PrismaModel>
  }

  export type NestedEnumSexFilter<$PrismaModel = never> = {
    equals?: $Enums.Sex | EnumSexFieldRefInput<$PrismaModel>
    in?: $Enums.Sex[] | ListEnumSexFieldRefInput<$PrismaModel>
    notIn?: $Enums.Sex[] | ListEnumSexFieldRefInput<$PrismaModel>
    not?: NestedEnumSexFilter<$PrismaModel> | $Enums.Sex
  }

  export type NestedEnumEthnicityFilter<$PrismaModel = never> = {
    equals?: $Enums.Ethnicity | EnumEthnicityFieldRefInput<$PrismaModel>
    in?: $Enums.Ethnicity[] | ListEnumEthnicityFieldRefInput<$PrismaModel>
    notIn?: $Enums.Ethnicity[] | ListEnumEthnicityFieldRefInput<$PrismaModel>
    not?: NestedEnumEthnicityFilter<$PrismaModel> | $Enums.Ethnicity
  }

  export type NestedEnumBasedOnFilter<$PrismaModel = never> = {
    equals?: $Enums.BasedOn | EnumBasedOnFieldRefInput<$PrismaModel>
    in?: $Enums.BasedOn[] | ListEnumBasedOnFieldRefInput<$PrismaModel>
    notIn?: $Enums.BasedOn[] | ListEnumBasedOnFieldRefInput<$PrismaModel>
    not?: NestedEnumBasedOnFilter<$PrismaModel> | $Enums.BasedOn
  }

  export type NestedEnumEyeColorFilter<$PrismaModel = never> = {
    equals?: $Enums.EyeColor | EnumEyeColorFieldRefInput<$PrismaModel>
    in?: $Enums.EyeColor[] | ListEnumEyeColorFieldRefInput<$PrismaModel>
    notIn?: $Enums.EyeColor[] | ListEnumEyeColorFieldRefInput<$PrismaModel>
    not?: NestedEnumEyeColorFilter<$PrismaModel> | $Enums.EyeColor
  }

  export type NestedBoolFilter<$PrismaModel = never> = {
    equals?: boolean | BooleanFieldRefInput<$PrismaModel>
    not?: NestedBoolFilter<$PrismaModel> | boolean
  }

  export type NestedEnumStatusFilter<$PrismaModel = never> = {
    equals?: $Enums.Status | EnumStatusFieldRefInput<$PrismaModel>
    in?: $Enums.Status[] | ListEnumStatusFieldRefInput<$PrismaModel>
    notIn?: $Enums.Status[] | ListEnumStatusFieldRefInput<$PrismaModel>
    not?: NestedEnumStatusFilter<$PrismaModel> | $Enums.Status
  }

  export type NestedDateTimeFilter<$PrismaModel = never> = {
    equals?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    in?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel>
    notIn?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel>
    lt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    lte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    not?: NestedDateTimeFilter<$PrismaModel> | Date | string
  }

  export type NestedEnumSexWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.Sex | EnumSexFieldRefInput<$PrismaModel>
    in?: $Enums.Sex[] | ListEnumSexFieldRefInput<$PrismaModel>
    notIn?: $Enums.Sex[] | ListEnumSexFieldRefInput<$PrismaModel>
    not?: NestedEnumSexWithAggregatesFilter<$PrismaModel> | $Enums.Sex
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumSexFilter<$PrismaModel>
    _max?: NestedEnumSexFilter<$PrismaModel>
  }

  export type NestedEnumEthnicityWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.Ethnicity | EnumEthnicityFieldRefInput<$PrismaModel>
    in?: $Enums.Ethnicity[] | ListEnumEthnicityFieldRefInput<$PrismaModel>
    notIn?: $Enums.Ethnicity[] | ListEnumEthnicityFieldRefInput<$PrismaModel>
    not?: NestedEnumEthnicityWithAggregatesFilter<$PrismaModel> | $Enums.Ethnicity
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumEthnicityFilter<$PrismaModel>
    _max?: NestedEnumEthnicityFilter<$PrismaModel>
  }

  export type NestedEnumBasedOnWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.BasedOn | EnumBasedOnFieldRefInput<$PrismaModel>
    in?: $Enums.BasedOn[] | ListEnumBasedOnFieldRefInput<$PrismaModel>
    notIn?: $Enums.BasedOn[] | ListEnumBasedOnFieldRefInput<$PrismaModel>
    not?: NestedEnumBasedOnWithAggregatesFilter<$PrismaModel> | $Enums.BasedOn
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumBasedOnFilter<$PrismaModel>
    _max?: NestedEnumBasedOnFilter<$PrismaModel>
  }

  export type NestedEnumEyeColorWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.EyeColor | EnumEyeColorFieldRefInput<$PrismaModel>
    in?: $Enums.EyeColor[] | ListEnumEyeColorFieldRefInput<$PrismaModel>
    notIn?: $Enums.EyeColor[] | ListEnumEyeColorFieldRefInput<$PrismaModel>
    not?: NestedEnumEyeColorWithAggregatesFilter<$PrismaModel> | $Enums.EyeColor
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumEyeColorFilter<$PrismaModel>
    _max?: NestedEnumEyeColorFilter<$PrismaModel>
  }

  export type NestedBoolWithAggregatesFilter<$PrismaModel = never> = {
    equals?: boolean | BooleanFieldRefInput<$PrismaModel>
    not?: NestedBoolWithAggregatesFilter<$PrismaModel> | boolean
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedBoolFilter<$PrismaModel>
    _max?: NestedBoolFilter<$PrismaModel>
  }

  export type NestedEnumStatusWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.Status | EnumStatusFieldRefInput<$PrismaModel>
    in?: $Enums.Status[] | ListEnumStatusFieldRefInput<$PrismaModel>
    notIn?: $Enums.Status[] | ListEnumStatusFieldRefInput<$PrismaModel>
    not?: NestedEnumStatusWithAggregatesFilter<$PrismaModel> | $Enums.Status
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumStatusFilter<$PrismaModel>
    _max?: NestedEnumStatusFilter<$PrismaModel>
  }
  export type NestedJsonNullableFilter<$PrismaModel = never> = 
    | PatchUndefined<
        Either<Required<NestedJsonNullableFilterBase<$PrismaModel>>, Exclude<keyof Required<NestedJsonNullableFilterBase<$PrismaModel>>, 'path'>>,
        Required<NestedJsonNullableFilterBase<$PrismaModel>>
      >
    | OptionalFlat<Omit<Required<NestedJsonNullableFilterBase<$PrismaModel>>, 'path'>>

  export type NestedJsonNullableFilterBase<$PrismaModel = never> = {
    equals?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | JsonNullValueFilter
    path?: string[]
    mode?: QueryMode | EnumQueryModeFieldRefInput<$PrismaModel>
    string_contains?: string | StringFieldRefInput<$PrismaModel>
    string_starts_with?: string | StringFieldRefInput<$PrismaModel>
    string_ends_with?: string | StringFieldRefInput<$PrismaModel>
    array_starts_with?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | null
    array_ends_with?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | null
    array_contains?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | null
    lt?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    lte?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    gt?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    gte?: InputJsonValue | JsonFieldRefInput<$PrismaModel>
    not?: InputJsonValue | JsonFieldRefInput<$PrismaModel> | JsonNullValueFilter
  }

  export type NestedDateTimeWithAggregatesFilter<$PrismaModel = never> = {
    equals?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    in?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel>
    notIn?: Date[] | string[] | ListDateTimeFieldRefInput<$PrismaModel>
    lt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    lte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gt?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    gte?: Date | string | DateTimeFieldRefInput<$PrismaModel>
    not?: NestedDateTimeWithAggregatesFilter<$PrismaModel> | Date | string
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedDateTimeFilter<$PrismaModel>
    _max?: NestedDateTimeFilter<$PrismaModel>
  }

  export type NestedEnumImageFormatFilter<$PrismaModel = never> = {
    equals?: $Enums.ImageFormat | EnumImageFormatFieldRefInput<$PrismaModel>
    in?: $Enums.ImageFormat[] | ListEnumImageFormatFieldRefInput<$PrismaModel>
    notIn?: $Enums.ImageFormat[] | ListEnumImageFormatFieldRefInput<$PrismaModel>
    not?: NestedEnumImageFormatFilter<$PrismaModel> | $Enums.ImageFormat
  }

  export type NestedEnumImageSizeFilter<$PrismaModel = never> = {
    equals?: $Enums.ImageSize | EnumImageSizeFieldRefInput<$PrismaModel>
    in?: $Enums.ImageSize[] | ListEnumImageSizeFieldRefInput<$PrismaModel>
    notIn?: $Enums.ImageSize[] | ListEnumImageSizeFieldRefInput<$PrismaModel>
    not?: NestedEnumImageSizeFilter<$PrismaModel> | $Enums.ImageSize
  }

  export type NestedIntNullableWithAggregatesFilter<$PrismaModel = never> = {
    equals?: number | IntFieldRefInput<$PrismaModel> | null
    in?: number[] | ListIntFieldRefInput<$PrismaModel> | null
    notIn?: number[] | ListIntFieldRefInput<$PrismaModel> | null
    lt?: number | IntFieldRefInput<$PrismaModel>
    lte?: number | IntFieldRefInput<$PrismaModel>
    gt?: number | IntFieldRefInput<$PrismaModel>
    gte?: number | IntFieldRefInput<$PrismaModel>
    not?: NestedIntNullableWithAggregatesFilter<$PrismaModel> | number | null
    _count?: NestedIntNullableFilter<$PrismaModel>
    _avg?: NestedFloatNullableFilter<$PrismaModel>
    _sum?: NestedIntNullableFilter<$PrismaModel>
    _min?: NestedIntNullableFilter<$PrismaModel>
    _max?: NestedIntNullableFilter<$PrismaModel>
  }

  export type NestedFloatNullableFilter<$PrismaModel = never> = {
    equals?: number | FloatFieldRefInput<$PrismaModel> | null
    in?: number[] | ListFloatFieldRefInput<$PrismaModel> | null
    notIn?: number[] | ListFloatFieldRefInput<$PrismaModel> | null
    lt?: number | FloatFieldRefInput<$PrismaModel>
    lte?: number | FloatFieldRefInput<$PrismaModel>
    gt?: number | FloatFieldRefInput<$PrismaModel>
    gte?: number | FloatFieldRefInput<$PrismaModel>
    not?: NestedFloatNullableFilter<$PrismaModel> | number | null
  }

  export type NestedEnumImageFormatWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.ImageFormat | EnumImageFormatFieldRefInput<$PrismaModel>
    in?: $Enums.ImageFormat[] | ListEnumImageFormatFieldRefInput<$PrismaModel>
    notIn?: $Enums.ImageFormat[] | ListEnumImageFormatFieldRefInput<$PrismaModel>
    not?: NestedEnumImageFormatWithAggregatesFilter<$PrismaModel> | $Enums.ImageFormat
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumImageFormatFilter<$PrismaModel>
    _max?: NestedEnumImageFormatFilter<$PrismaModel>
  }

  export type NestedEnumImageSizeWithAggregatesFilter<$PrismaModel = never> = {
    equals?: $Enums.ImageSize | EnumImageSizeFieldRefInput<$PrismaModel>
    in?: $Enums.ImageSize[] | ListEnumImageSizeFieldRefInput<$PrismaModel>
    notIn?: $Enums.ImageSize[] | ListEnumImageSizeFieldRefInput<$PrismaModel>
    not?: NestedEnumImageSizeWithAggregatesFilter<$PrismaModel> | $Enums.ImageSize
    _count?: NestedIntFilter<$PrismaModel>
    _min?: NestedEnumImageSizeFilter<$PrismaModel>
    _max?: NestedEnumImageSizeFilter<$PrismaModel>
  }

  export type TrainingModelsCreateWithoutUserInput = {
    pid: string
    name: string
    age: number
    sex: $Enums.Sex
    ethnicity: $Enums.Ethnicity
    basedOn: $Enums.BasedOn
    eye_color: $Enums.EyeColor
    bald?: boolean
    steps: number
    create_mask: boolean
    is_style: boolean
    trigger_word: string
    tensor_path?: string | null
    thumbnail?: string | null
    training_status: $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: string | null
    s3_key: string
    is_verified?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
    Images?: ImagesCreateNestedManyWithoutTraining_modelInput
  }

  export type TrainingModelsUncheckedCreateWithoutUserInput = {
    id?: number
    pid: string
    name: string
    age: number
    sex: $Enums.Sex
    ethnicity: $Enums.Ethnicity
    basedOn: $Enums.BasedOn
    eye_color: $Enums.EyeColor
    bald?: boolean
    steps: number
    create_mask: boolean
    is_style: boolean
    trigger_word: string
    tensor_path?: string | null
    thumbnail?: string | null
    training_status: $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: string | null
    s3_key: string
    is_verified?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
    Images?: ImagesUncheckedCreateNestedManyWithoutTraining_modelInput
  }

  export type TrainingModelsCreateOrConnectWithoutUserInput = {
    where: TrainingModelsWhereUniqueInput
    create: XOR<TrainingModelsCreateWithoutUserInput, TrainingModelsUncheckedCreateWithoutUserInput>
  }

  export type TrainingModelsCreateManyUserInputEnvelope = {
    data: TrainingModelsCreateManyUserInput | TrainingModelsCreateManyUserInput[]
    skipDuplicates?: boolean
  }

  export type UserCreditsCreateWithoutUserInput = {
    pid: string
    credit_amount?: number
    model_amount?: number
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type UserCreditsUncheckedCreateWithoutUserInput = {
    id?: number
    pid: string
    credit_amount?: number
    model_amount?: number
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type UserCreditsCreateOrConnectWithoutUserInput = {
    where: UserCreditsWhereUniqueInput
    create: XOR<UserCreditsCreateWithoutUserInput, UserCreditsUncheckedCreateWithoutUserInput>
  }

  export type UserCreditsCreateManyUserInputEnvelope = {
    data: UserCreditsCreateManyUserInput | UserCreditsCreateManyUserInput[]
    skipDuplicates?: boolean
  }

  export type ImagesCreateWithoutUserInput = {
    pid: string
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
    training_model: TrainingModelsCreateNestedOneWithoutImagesInput
    pack?: PacksCreateNestedOneWithoutImagesInput
  }

  export type ImagesUncheckedCreateWithoutUserInput = {
    id?: number
    pid: string
    training_model_id: number
    pack_id?: number | null
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type ImagesCreateOrConnectWithoutUserInput = {
    where: ImagesWhereUniqueInput
    create: XOR<ImagesCreateWithoutUserInput, ImagesUncheckedCreateWithoutUserInput>
  }

  export type ImagesCreateManyUserInputEnvelope = {
    data: ImagesCreateManyUserInput | ImagesCreateManyUserInput[]
    skipDuplicates?: boolean
  }

  export type TransactionsCreateWithoutUserInput = {
    pid: string
    credit_amount: number
    model_amount: number
    currency: string
    payment_id: string
    order_id: string
    plan: string
    status: $Enums.Status
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type TransactionsUncheckedCreateWithoutUserInput = {
    id?: number
    pid: string
    credit_amount: number
    model_amount: number
    currency: string
    payment_id: string
    order_id: string
    plan: string
    status: $Enums.Status
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type TransactionsCreateOrConnectWithoutUserInput = {
    where: TransactionsWhereUniqueInput
    create: XOR<TransactionsCreateWithoutUserInput, TransactionsUncheckedCreateWithoutUserInput>
  }

  export type TransactionsCreateManyUserInputEnvelope = {
    data: TransactionsCreateManyUserInput | TransactionsCreateManyUserInput[]
    skipDuplicates?: boolean
  }

  export type TrainingModelsUpsertWithWhereUniqueWithoutUserInput = {
    where: TrainingModelsWhereUniqueInput
    update: XOR<TrainingModelsUpdateWithoutUserInput, TrainingModelsUncheckedUpdateWithoutUserInput>
    create: XOR<TrainingModelsCreateWithoutUserInput, TrainingModelsUncheckedCreateWithoutUserInput>
  }

  export type TrainingModelsUpdateWithWhereUniqueWithoutUserInput = {
    where: TrainingModelsWhereUniqueInput
    data: XOR<TrainingModelsUpdateWithoutUserInput, TrainingModelsUncheckedUpdateWithoutUserInput>
  }

  export type TrainingModelsUpdateManyWithWhereWithoutUserInput = {
    where: TrainingModelsScalarWhereInput
    data: XOR<TrainingModelsUpdateManyMutationInput, TrainingModelsUncheckedUpdateManyWithoutUserInput>
  }

  export type TrainingModelsScalarWhereInput = {
    AND?: TrainingModelsScalarWhereInput | TrainingModelsScalarWhereInput[]
    OR?: TrainingModelsScalarWhereInput[]
    NOT?: TrainingModelsScalarWhereInput | TrainingModelsScalarWhereInput[]
    id?: IntFilter<"TrainingModels"> | number
    pid?: UuidFilter<"TrainingModels"> | string
    user_id?: IntFilter<"TrainingModels"> | number
    name?: StringFilter<"TrainingModels"> | string
    age?: IntFilter<"TrainingModels"> | number
    sex?: EnumSexFilter<"TrainingModels"> | $Enums.Sex
    ethnicity?: EnumEthnicityFilter<"TrainingModels"> | $Enums.Ethnicity
    basedOn?: EnumBasedOnFilter<"TrainingModels"> | $Enums.BasedOn
    eye_color?: EnumEyeColorFilter<"TrainingModels"> | $Enums.EyeColor
    bald?: BoolFilter<"TrainingModels"> | boolean
    steps?: IntFilter<"TrainingModels"> | number
    create_mask?: BoolFilter<"TrainingModels"> | boolean
    is_style?: BoolFilter<"TrainingModels"> | boolean
    trigger_word?: StringFilter<"TrainingModels"> | string
    tensor_path?: StringNullableFilter<"TrainingModels"> | string | null
    thumbnail?: StringNullableFilter<"TrainingModels"> | string | null
    training_status?: EnumStatusFilter<"TrainingModels"> | $Enums.Status
    fal_output?: JsonNullableFilter<"TrainingModels">
    training_images?: JsonNullableFilter<"TrainingModels">
    fal_ai_request_id?: StringNullableFilter<"TrainingModels"> | string | null
    s3_key?: StringFilter<"TrainingModels"> | string
    is_verified?: BoolFilter<"TrainingModels"> | boolean
    deleted_at?: DateTimeNullableFilter<"TrainingModels"> | Date | string | null
    created_at?: DateTimeFilter<"TrainingModels"> | Date | string
    updated_at?: DateTimeFilter<"TrainingModels"> | Date | string
  }

  export type UserCreditsUpsertWithWhereUniqueWithoutUserInput = {
    where: UserCreditsWhereUniqueInput
    update: XOR<UserCreditsUpdateWithoutUserInput, UserCreditsUncheckedUpdateWithoutUserInput>
    create: XOR<UserCreditsCreateWithoutUserInput, UserCreditsUncheckedCreateWithoutUserInput>
  }

  export type UserCreditsUpdateWithWhereUniqueWithoutUserInput = {
    where: UserCreditsWhereUniqueInput
    data: XOR<UserCreditsUpdateWithoutUserInput, UserCreditsUncheckedUpdateWithoutUserInput>
  }

  export type UserCreditsUpdateManyWithWhereWithoutUserInput = {
    where: UserCreditsScalarWhereInput
    data: XOR<UserCreditsUpdateManyMutationInput, UserCreditsUncheckedUpdateManyWithoutUserInput>
  }

  export type UserCreditsScalarWhereInput = {
    AND?: UserCreditsScalarWhereInput | UserCreditsScalarWhereInput[]
    OR?: UserCreditsScalarWhereInput[]
    NOT?: UserCreditsScalarWhereInput | UserCreditsScalarWhereInput[]
    id?: IntFilter<"UserCredits"> | number
    pid?: UuidFilter<"UserCredits"> | string
    user_id?: IntFilter<"UserCredits"> | number
    credit_amount?: IntFilter<"UserCredits"> | number
    model_amount?: IntFilter<"UserCredits"> | number
    created_at?: DateTimeFilter<"UserCredits"> | Date | string
    updated_at?: DateTimeFilter<"UserCredits"> | Date | string
  }

  export type ImagesUpsertWithWhereUniqueWithoutUserInput = {
    where: ImagesWhereUniqueInput
    update: XOR<ImagesUpdateWithoutUserInput, ImagesUncheckedUpdateWithoutUserInput>
    create: XOR<ImagesCreateWithoutUserInput, ImagesUncheckedCreateWithoutUserInput>
  }

  export type ImagesUpdateWithWhereUniqueWithoutUserInput = {
    where: ImagesWhereUniqueInput
    data: XOR<ImagesUpdateWithoutUserInput, ImagesUncheckedUpdateWithoutUserInput>
  }

  export type ImagesUpdateManyWithWhereWithoutUserInput = {
    where: ImagesScalarWhereInput
    data: XOR<ImagesUpdateManyMutationInput, ImagesUncheckedUpdateManyWithoutUserInput>
  }

  export type ImagesScalarWhereInput = {
    AND?: ImagesScalarWhereInput | ImagesScalarWhereInput[]
    OR?: ImagesScalarWhereInput[]
    NOT?: ImagesScalarWhereInput | ImagesScalarWhereInput[]
    id?: IntFilter<"Images"> | number
    pid?: UuidFilter<"Images"> | string
    user_id?: IntFilter<"Images"> | number
    training_model_id?: IntFilter<"Images"> | number
    pack_id?: IntNullableFilter<"Images"> | number | null
    user_prompt?: StringFilter<"Images"> | string
    sys_prompt?: StringFilter<"Images"> | string
    alt?: StringFilter<"Images"> | string
    num_inference_steps?: IntFilter<"Images"> | number
    content_type?: EnumImageFormatFilter<"Images"> | $Enums.ImageFormat
    status?: EnumStatusFilter<"Images"> | $Enums.Status
    image_size?: EnumImageSizeFilter<"Images"> | $Enums.ImageSize
    fal_ai_request_id?: StringNullableFilter<"Images"> | string | null
    width?: IntNullableFilter<"Images"> | number | null
    height?: IntNullableFilter<"Images"> | number | null
    image_url?: StringNullableFilter<"Images"> | string | null
    image_url_s3?: StringNullableFilter<"Images"> | string | null
    is_favorite?: BoolFilter<"Images"> | boolean
    deleted_at?: DateTimeNullableFilter<"Images"> | Date | string | null
    created_at?: DateTimeFilter<"Images"> | Date | string
    updated_at?: DateTimeFilter<"Images"> | Date | string
  }

  export type TransactionsUpsertWithWhereUniqueWithoutUserInput = {
    where: TransactionsWhereUniqueInput
    update: XOR<TransactionsUpdateWithoutUserInput, TransactionsUncheckedUpdateWithoutUserInput>
    create: XOR<TransactionsCreateWithoutUserInput, TransactionsUncheckedCreateWithoutUserInput>
  }

  export type TransactionsUpdateWithWhereUniqueWithoutUserInput = {
    where: TransactionsWhereUniqueInput
    data: XOR<TransactionsUpdateWithoutUserInput, TransactionsUncheckedUpdateWithoutUserInput>
  }

  export type TransactionsUpdateManyWithWhereWithoutUserInput = {
    where: TransactionsScalarWhereInput
    data: XOR<TransactionsUpdateManyMutationInput, TransactionsUncheckedUpdateManyWithoutUserInput>
  }

  export type TransactionsScalarWhereInput = {
    AND?: TransactionsScalarWhereInput | TransactionsScalarWhereInput[]
    OR?: TransactionsScalarWhereInput[]
    NOT?: TransactionsScalarWhereInput | TransactionsScalarWhereInput[]
    id?: IntFilter<"Transactions"> | number
    pid?: UuidFilter<"Transactions"> | string
    user_id?: IntFilter<"Transactions"> | number
    credit_amount?: IntFilter<"Transactions"> | number
    model_amount?: IntFilter<"Transactions"> | number
    currency?: StringFilter<"Transactions"> | string
    payment_id?: StringFilter<"Transactions"> | string
    order_id?: StringFilter<"Transactions"> | string
    plan?: StringFilter<"Transactions"> | string
    status?: EnumStatusFilter<"Transactions"> | $Enums.Status
    created_at?: DateTimeFilter<"Transactions"> | Date | string
    updated_at?: DateTimeFilter<"Transactions"> | Date | string
  }

  export type UsersCreateWithoutTrainingModelsInput = {
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
    UserCredits?: UserCreditsCreateNestedManyWithoutUserInput
    Images?: ImagesCreateNestedManyWithoutUserInput
    Transactions?: TransactionsCreateNestedManyWithoutUserInput
  }

  export type UsersUncheckedCreateWithoutTrainingModelsInput = {
    id?: number
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
    UserCredits?: UserCreditsUncheckedCreateNestedManyWithoutUserInput
    Images?: ImagesUncheckedCreateNestedManyWithoutUserInput
    Transactions?: TransactionsUncheckedCreateNestedManyWithoutUserInput
  }

  export type UsersCreateOrConnectWithoutTrainingModelsInput = {
    where: UsersWhereUniqueInput
    create: XOR<UsersCreateWithoutTrainingModelsInput, UsersUncheckedCreateWithoutTrainingModelsInput>
  }

  export type ImagesCreateWithoutTraining_modelInput = {
    pid: string
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
    user: UsersCreateNestedOneWithoutImagesInput
    pack?: PacksCreateNestedOneWithoutImagesInput
  }

  export type ImagesUncheckedCreateWithoutTraining_modelInput = {
    id?: number
    pid: string
    user_id: number
    pack_id?: number | null
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type ImagesCreateOrConnectWithoutTraining_modelInput = {
    where: ImagesWhereUniqueInput
    create: XOR<ImagesCreateWithoutTraining_modelInput, ImagesUncheckedCreateWithoutTraining_modelInput>
  }

  export type ImagesCreateManyTraining_modelInputEnvelope = {
    data: ImagesCreateManyTraining_modelInput | ImagesCreateManyTraining_modelInput[]
    skipDuplicates?: boolean
  }

  export type UsersUpsertWithoutTrainingModelsInput = {
    update: XOR<UsersUpdateWithoutTrainingModelsInput, UsersUncheckedUpdateWithoutTrainingModelsInput>
    create: XOR<UsersCreateWithoutTrainingModelsInput, UsersUncheckedCreateWithoutTrainingModelsInput>
    where?: UsersWhereInput
  }

  export type UsersUpdateToOneWithWhereWithoutTrainingModelsInput = {
    where?: UsersWhereInput
    data: XOR<UsersUpdateWithoutTrainingModelsInput, UsersUncheckedUpdateWithoutTrainingModelsInput>
  }

  export type UsersUpdateWithoutTrainingModelsInput = {
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    UserCredits?: UserCreditsUpdateManyWithoutUserNestedInput
    Images?: ImagesUpdateManyWithoutUserNestedInput
    Transactions?: TransactionsUpdateManyWithoutUserNestedInput
  }

  export type UsersUncheckedUpdateWithoutTrainingModelsInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    UserCredits?: UserCreditsUncheckedUpdateManyWithoutUserNestedInput
    Images?: ImagesUncheckedUpdateManyWithoutUserNestedInput
    Transactions?: TransactionsUncheckedUpdateManyWithoutUserNestedInput
  }

  export type ImagesUpsertWithWhereUniqueWithoutTraining_modelInput = {
    where: ImagesWhereUniqueInput
    update: XOR<ImagesUpdateWithoutTraining_modelInput, ImagesUncheckedUpdateWithoutTraining_modelInput>
    create: XOR<ImagesCreateWithoutTraining_modelInput, ImagesUncheckedCreateWithoutTraining_modelInput>
  }

  export type ImagesUpdateWithWhereUniqueWithoutTraining_modelInput = {
    where: ImagesWhereUniqueInput
    data: XOR<ImagesUpdateWithoutTraining_modelInput, ImagesUncheckedUpdateWithoutTraining_modelInput>
  }

  export type ImagesUpdateManyWithWhereWithoutTraining_modelInput = {
    where: ImagesScalarWhereInput
    data: XOR<ImagesUpdateManyMutationInput, ImagesUncheckedUpdateManyWithoutTraining_modelInput>
  }

  export type UsersCreateWithoutUserCreditsInput = {
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
    TrainingModels?: TrainingModelsCreateNestedManyWithoutUserInput
    Images?: ImagesCreateNestedManyWithoutUserInput
    Transactions?: TransactionsCreateNestedManyWithoutUserInput
  }

  export type UsersUncheckedCreateWithoutUserCreditsInput = {
    id?: number
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
    TrainingModels?: TrainingModelsUncheckedCreateNestedManyWithoutUserInput
    Images?: ImagesUncheckedCreateNestedManyWithoutUserInput
    Transactions?: TransactionsUncheckedCreateNestedManyWithoutUserInput
  }

  export type UsersCreateOrConnectWithoutUserCreditsInput = {
    where: UsersWhereUniqueInput
    create: XOR<UsersCreateWithoutUserCreditsInput, UsersUncheckedCreateWithoutUserCreditsInput>
  }

  export type UsersUpsertWithoutUserCreditsInput = {
    update: XOR<UsersUpdateWithoutUserCreditsInput, UsersUncheckedUpdateWithoutUserCreditsInput>
    create: XOR<UsersCreateWithoutUserCreditsInput, UsersUncheckedCreateWithoutUserCreditsInput>
    where?: UsersWhereInput
  }

  export type UsersUpdateToOneWithWhereWithoutUserCreditsInput = {
    where?: UsersWhereInput
    data: XOR<UsersUpdateWithoutUserCreditsInput, UsersUncheckedUpdateWithoutUserCreditsInput>
  }

  export type UsersUpdateWithoutUserCreditsInput = {
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    TrainingModels?: TrainingModelsUpdateManyWithoutUserNestedInput
    Images?: ImagesUpdateManyWithoutUserNestedInput
    Transactions?: TransactionsUpdateManyWithoutUserNestedInput
  }

  export type UsersUncheckedUpdateWithoutUserCreditsInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    TrainingModels?: TrainingModelsUncheckedUpdateManyWithoutUserNestedInput
    Images?: ImagesUncheckedUpdateManyWithoutUserNestedInput
    Transactions?: TransactionsUncheckedUpdateManyWithoutUserNestedInput
  }

  export type UsersCreateWithoutImagesInput = {
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
    TrainingModels?: TrainingModelsCreateNestedManyWithoutUserInput
    UserCredits?: UserCreditsCreateNestedManyWithoutUserInput
    Transactions?: TransactionsCreateNestedManyWithoutUserInput
  }

  export type UsersUncheckedCreateWithoutImagesInput = {
    id?: number
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
    TrainingModels?: TrainingModelsUncheckedCreateNestedManyWithoutUserInput
    UserCredits?: UserCreditsUncheckedCreateNestedManyWithoutUserInput
    Transactions?: TransactionsUncheckedCreateNestedManyWithoutUserInput
  }

  export type UsersCreateOrConnectWithoutImagesInput = {
    where: UsersWhereUniqueInput
    create: XOR<UsersCreateWithoutImagesInput, UsersUncheckedCreateWithoutImagesInput>
  }

  export type TrainingModelsCreateWithoutImagesInput = {
    pid: string
    name: string
    age: number
    sex: $Enums.Sex
    ethnicity: $Enums.Ethnicity
    basedOn: $Enums.BasedOn
    eye_color: $Enums.EyeColor
    bald?: boolean
    steps: number
    create_mask: boolean
    is_style: boolean
    trigger_word: string
    tensor_path?: string | null
    thumbnail?: string | null
    training_status: $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: string | null
    s3_key: string
    is_verified?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
    user: UsersCreateNestedOneWithoutTrainingModelsInput
  }

  export type TrainingModelsUncheckedCreateWithoutImagesInput = {
    id?: number
    pid: string
    user_id: number
    name: string
    age: number
    sex: $Enums.Sex
    ethnicity: $Enums.Ethnicity
    basedOn: $Enums.BasedOn
    eye_color: $Enums.EyeColor
    bald?: boolean
    steps: number
    create_mask: boolean
    is_style: boolean
    trigger_word: string
    tensor_path?: string | null
    thumbnail?: string | null
    training_status: $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: string | null
    s3_key: string
    is_verified?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type TrainingModelsCreateOrConnectWithoutImagesInput = {
    where: TrainingModelsWhereUniqueInput
    create: XOR<TrainingModelsCreateWithoutImagesInput, TrainingModelsUncheckedCreateWithoutImagesInput>
  }

  export type PacksCreateWithoutImagesInput = {
    pid: string
    name: string
    description: string
    pack_prompts: string
    credits: number
    amount: number
    image_url: string
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type PacksUncheckedCreateWithoutImagesInput = {
    id?: number
    pid: string
    name: string
    description: string
    pack_prompts: string
    credits: number
    amount: number
    image_url: string
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type PacksCreateOrConnectWithoutImagesInput = {
    where: PacksWhereUniqueInput
    create: XOR<PacksCreateWithoutImagesInput, PacksUncheckedCreateWithoutImagesInput>
  }

  export type UsersUpsertWithoutImagesInput = {
    update: XOR<UsersUpdateWithoutImagesInput, UsersUncheckedUpdateWithoutImagesInput>
    create: XOR<UsersCreateWithoutImagesInput, UsersUncheckedCreateWithoutImagesInput>
    where?: UsersWhereInput
  }

  export type UsersUpdateToOneWithWhereWithoutImagesInput = {
    where?: UsersWhereInput
    data: XOR<UsersUpdateWithoutImagesInput, UsersUncheckedUpdateWithoutImagesInput>
  }

  export type UsersUpdateWithoutImagesInput = {
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    TrainingModels?: TrainingModelsUpdateManyWithoutUserNestedInput
    UserCredits?: UserCreditsUpdateManyWithoutUserNestedInput
    Transactions?: TransactionsUpdateManyWithoutUserNestedInput
  }

  export type UsersUncheckedUpdateWithoutImagesInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    TrainingModels?: TrainingModelsUncheckedUpdateManyWithoutUserNestedInput
    UserCredits?: UserCreditsUncheckedUpdateManyWithoutUserNestedInput
    Transactions?: TransactionsUncheckedUpdateManyWithoutUserNestedInput
  }

  export type TrainingModelsUpsertWithoutImagesInput = {
    update: XOR<TrainingModelsUpdateWithoutImagesInput, TrainingModelsUncheckedUpdateWithoutImagesInput>
    create: XOR<TrainingModelsCreateWithoutImagesInput, TrainingModelsUncheckedCreateWithoutImagesInput>
    where?: TrainingModelsWhereInput
  }

  export type TrainingModelsUpdateToOneWithWhereWithoutImagesInput = {
    where?: TrainingModelsWhereInput
    data: XOR<TrainingModelsUpdateWithoutImagesInput, TrainingModelsUncheckedUpdateWithoutImagesInput>
  }

  export type TrainingModelsUpdateWithoutImagesInput = {
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    age?: IntFieldUpdateOperationsInput | number
    sex?: EnumSexFieldUpdateOperationsInput | $Enums.Sex
    ethnicity?: EnumEthnicityFieldUpdateOperationsInput | $Enums.Ethnicity
    basedOn?: EnumBasedOnFieldUpdateOperationsInput | $Enums.BasedOn
    eye_color?: EnumEyeColorFieldUpdateOperationsInput | $Enums.EyeColor
    bald?: BoolFieldUpdateOperationsInput | boolean
    steps?: IntFieldUpdateOperationsInput | number
    create_mask?: BoolFieldUpdateOperationsInput | boolean
    is_style?: BoolFieldUpdateOperationsInput | boolean
    trigger_word?: StringFieldUpdateOperationsInput | string
    tensor_path?: NullableStringFieldUpdateOperationsInput | string | null
    thumbnail?: NullableStringFieldUpdateOperationsInput | string | null
    training_status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    s3_key?: StringFieldUpdateOperationsInput | string
    is_verified?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    user?: UsersUpdateOneRequiredWithoutTrainingModelsNestedInput
  }

  export type TrainingModelsUncheckedUpdateWithoutImagesInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    name?: StringFieldUpdateOperationsInput | string
    age?: IntFieldUpdateOperationsInput | number
    sex?: EnumSexFieldUpdateOperationsInput | $Enums.Sex
    ethnicity?: EnumEthnicityFieldUpdateOperationsInput | $Enums.Ethnicity
    basedOn?: EnumBasedOnFieldUpdateOperationsInput | $Enums.BasedOn
    eye_color?: EnumEyeColorFieldUpdateOperationsInput | $Enums.EyeColor
    bald?: BoolFieldUpdateOperationsInput | boolean
    steps?: IntFieldUpdateOperationsInput | number
    create_mask?: BoolFieldUpdateOperationsInput | boolean
    is_style?: BoolFieldUpdateOperationsInput | boolean
    trigger_word?: StringFieldUpdateOperationsInput | string
    tensor_path?: NullableStringFieldUpdateOperationsInput | string | null
    thumbnail?: NullableStringFieldUpdateOperationsInput | string | null
    training_status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    s3_key?: StringFieldUpdateOperationsInput | string
    is_verified?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type PacksUpsertWithoutImagesInput = {
    update: XOR<PacksUpdateWithoutImagesInput, PacksUncheckedUpdateWithoutImagesInput>
    create: XOR<PacksCreateWithoutImagesInput, PacksUncheckedCreateWithoutImagesInput>
    where?: PacksWhereInput
  }

  export type PacksUpdateToOneWithWhereWithoutImagesInput = {
    where?: PacksWhereInput
    data: XOR<PacksUpdateWithoutImagesInput, PacksUncheckedUpdateWithoutImagesInput>
  }

  export type PacksUpdateWithoutImagesInput = {
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    description?: StringFieldUpdateOperationsInput | string
    pack_prompts?: StringFieldUpdateOperationsInput | string
    credits?: IntFieldUpdateOperationsInput | number
    amount?: IntFieldUpdateOperationsInput | number
    image_url?: StringFieldUpdateOperationsInput | string
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type PacksUncheckedUpdateWithoutImagesInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    description?: StringFieldUpdateOperationsInput | string
    pack_prompts?: StringFieldUpdateOperationsInput | string
    credits?: IntFieldUpdateOperationsInput | number
    amount?: IntFieldUpdateOperationsInput | number
    image_url?: StringFieldUpdateOperationsInput | string
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type UsersCreateWithoutTransactionsInput = {
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
    TrainingModels?: TrainingModelsCreateNestedManyWithoutUserInput
    UserCredits?: UserCreditsCreateNestedManyWithoutUserInput
    Images?: ImagesCreateNestedManyWithoutUserInput
  }

  export type UsersUncheckedCreateWithoutTransactionsInput = {
    id?: number
    pid: string
    email: string
    password: string
    api_key: string
    name: string
    reset_token?: string | null
    reset_sent_at?: Date | string | null
    email_verification_token?: string | null
    email_verification_sent_at?: Date | string | null
    email_verified_at?: Date | string | null
    magicLink_token?: string | null
    magicLink_expiration?: Date | string | null
    TrainingModels?: TrainingModelsUncheckedCreateNestedManyWithoutUserInput
    UserCredits?: UserCreditsUncheckedCreateNestedManyWithoutUserInput
    Images?: ImagesUncheckedCreateNestedManyWithoutUserInput
  }

  export type UsersCreateOrConnectWithoutTransactionsInput = {
    where: UsersWhereUniqueInput
    create: XOR<UsersCreateWithoutTransactionsInput, UsersUncheckedCreateWithoutTransactionsInput>
  }

  export type UsersUpsertWithoutTransactionsInput = {
    update: XOR<UsersUpdateWithoutTransactionsInput, UsersUncheckedUpdateWithoutTransactionsInput>
    create: XOR<UsersCreateWithoutTransactionsInput, UsersUncheckedCreateWithoutTransactionsInput>
    where?: UsersWhereInput
  }

  export type UsersUpdateToOneWithWhereWithoutTransactionsInput = {
    where?: UsersWhereInput
    data: XOR<UsersUpdateWithoutTransactionsInput, UsersUncheckedUpdateWithoutTransactionsInput>
  }

  export type UsersUpdateWithoutTransactionsInput = {
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    TrainingModels?: TrainingModelsUpdateManyWithoutUserNestedInput
    UserCredits?: UserCreditsUpdateManyWithoutUserNestedInput
    Images?: ImagesUpdateManyWithoutUserNestedInput
  }

  export type UsersUncheckedUpdateWithoutTransactionsInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    email?: StringFieldUpdateOperationsInput | string
    password?: StringFieldUpdateOperationsInput | string
    api_key?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    reset_token?: NullableStringFieldUpdateOperationsInput | string | null
    reset_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verification_token?: NullableStringFieldUpdateOperationsInput | string | null
    email_verification_sent_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    email_verified_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    magicLink_token?: NullableStringFieldUpdateOperationsInput | string | null
    magicLink_expiration?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    TrainingModels?: TrainingModelsUncheckedUpdateManyWithoutUserNestedInput
    UserCredits?: UserCreditsUncheckedUpdateManyWithoutUserNestedInput
    Images?: ImagesUncheckedUpdateManyWithoutUserNestedInput
  }

  export type ImagesCreateWithoutPackInput = {
    pid: string
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
    user: UsersCreateNestedOneWithoutImagesInput
    training_model: TrainingModelsCreateNestedOneWithoutImagesInput
  }

  export type ImagesUncheckedCreateWithoutPackInput = {
    id?: number
    pid: string
    user_id: number
    training_model_id: number
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type ImagesCreateOrConnectWithoutPackInput = {
    where: ImagesWhereUniqueInput
    create: XOR<ImagesCreateWithoutPackInput, ImagesUncheckedCreateWithoutPackInput>
  }

  export type ImagesCreateManyPackInputEnvelope = {
    data: ImagesCreateManyPackInput | ImagesCreateManyPackInput[]
    skipDuplicates?: boolean
  }

  export type ImagesUpsertWithWhereUniqueWithoutPackInput = {
    where: ImagesWhereUniqueInput
    update: XOR<ImagesUpdateWithoutPackInput, ImagesUncheckedUpdateWithoutPackInput>
    create: XOR<ImagesCreateWithoutPackInput, ImagesUncheckedCreateWithoutPackInput>
  }

  export type ImagesUpdateWithWhereUniqueWithoutPackInput = {
    where: ImagesWhereUniqueInput
    data: XOR<ImagesUpdateWithoutPackInput, ImagesUncheckedUpdateWithoutPackInput>
  }

  export type ImagesUpdateManyWithWhereWithoutPackInput = {
    where: ImagesScalarWhereInput
    data: XOR<ImagesUpdateManyMutationInput, ImagesUncheckedUpdateManyWithoutPackInput>
  }

  export type TrainingModelsCreateManyUserInput = {
    id?: number
    pid: string
    name: string
    age: number
    sex: $Enums.Sex
    ethnicity: $Enums.Ethnicity
    basedOn: $Enums.BasedOn
    eye_color: $Enums.EyeColor
    bald?: boolean
    steps: number
    create_mask: boolean
    is_style: boolean
    trigger_word: string
    tensor_path?: string | null
    thumbnail?: string | null
    training_status: $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: string | null
    s3_key: string
    is_verified?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type UserCreditsCreateManyUserInput = {
    id?: number
    pid: string
    credit_amount?: number
    model_amount?: number
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type ImagesCreateManyUserInput = {
    id?: number
    pid: string
    training_model_id: number
    pack_id?: number | null
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type TransactionsCreateManyUserInput = {
    id?: number
    pid: string
    credit_amount: number
    model_amount: number
    currency: string
    payment_id: string
    order_id: string
    plan: string
    status: $Enums.Status
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type TrainingModelsUpdateWithoutUserInput = {
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    age?: IntFieldUpdateOperationsInput | number
    sex?: EnumSexFieldUpdateOperationsInput | $Enums.Sex
    ethnicity?: EnumEthnicityFieldUpdateOperationsInput | $Enums.Ethnicity
    basedOn?: EnumBasedOnFieldUpdateOperationsInput | $Enums.BasedOn
    eye_color?: EnumEyeColorFieldUpdateOperationsInput | $Enums.EyeColor
    bald?: BoolFieldUpdateOperationsInput | boolean
    steps?: IntFieldUpdateOperationsInput | number
    create_mask?: BoolFieldUpdateOperationsInput | boolean
    is_style?: BoolFieldUpdateOperationsInput | boolean
    trigger_word?: StringFieldUpdateOperationsInput | string
    tensor_path?: NullableStringFieldUpdateOperationsInput | string | null
    thumbnail?: NullableStringFieldUpdateOperationsInput | string | null
    training_status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    s3_key?: StringFieldUpdateOperationsInput | string
    is_verified?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    Images?: ImagesUpdateManyWithoutTraining_modelNestedInput
  }

  export type TrainingModelsUncheckedUpdateWithoutUserInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    age?: IntFieldUpdateOperationsInput | number
    sex?: EnumSexFieldUpdateOperationsInput | $Enums.Sex
    ethnicity?: EnumEthnicityFieldUpdateOperationsInput | $Enums.Ethnicity
    basedOn?: EnumBasedOnFieldUpdateOperationsInput | $Enums.BasedOn
    eye_color?: EnumEyeColorFieldUpdateOperationsInput | $Enums.EyeColor
    bald?: BoolFieldUpdateOperationsInput | boolean
    steps?: IntFieldUpdateOperationsInput | number
    create_mask?: BoolFieldUpdateOperationsInput | boolean
    is_style?: BoolFieldUpdateOperationsInput | boolean
    trigger_word?: StringFieldUpdateOperationsInput | string
    tensor_path?: NullableStringFieldUpdateOperationsInput | string | null
    thumbnail?: NullableStringFieldUpdateOperationsInput | string | null
    training_status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    s3_key?: StringFieldUpdateOperationsInput | string
    is_verified?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    Images?: ImagesUncheckedUpdateManyWithoutTraining_modelNestedInput
  }

  export type TrainingModelsUncheckedUpdateManyWithoutUserInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    name?: StringFieldUpdateOperationsInput | string
    age?: IntFieldUpdateOperationsInput | number
    sex?: EnumSexFieldUpdateOperationsInput | $Enums.Sex
    ethnicity?: EnumEthnicityFieldUpdateOperationsInput | $Enums.Ethnicity
    basedOn?: EnumBasedOnFieldUpdateOperationsInput | $Enums.BasedOn
    eye_color?: EnumEyeColorFieldUpdateOperationsInput | $Enums.EyeColor
    bald?: BoolFieldUpdateOperationsInput | boolean
    steps?: IntFieldUpdateOperationsInput | number
    create_mask?: BoolFieldUpdateOperationsInput | boolean
    is_style?: BoolFieldUpdateOperationsInput | boolean
    trigger_word?: StringFieldUpdateOperationsInput | string
    tensor_path?: NullableStringFieldUpdateOperationsInput | string | null
    thumbnail?: NullableStringFieldUpdateOperationsInput | string | null
    training_status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    fal_output?: NullableJsonNullValueInput | InputJsonValue
    training_images?: NullableJsonNullValueInput | InputJsonValue
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    s3_key?: StringFieldUpdateOperationsInput | string
    is_verified?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type UserCreditsUpdateWithoutUserInput = {
    pid?: StringFieldUpdateOperationsInput | string
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type UserCreditsUncheckedUpdateWithoutUserInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type UserCreditsUncheckedUpdateManyWithoutUserInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type ImagesUpdateWithoutUserInput = {
    pid?: StringFieldUpdateOperationsInput | string
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    training_model?: TrainingModelsUpdateOneRequiredWithoutImagesNestedInput
    pack?: PacksUpdateOneWithoutImagesNestedInput
  }

  export type ImagesUncheckedUpdateWithoutUserInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    training_model_id?: IntFieldUpdateOperationsInput | number
    pack_id?: NullableIntFieldUpdateOperationsInput | number | null
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type ImagesUncheckedUpdateManyWithoutUserInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    training_model_id?: IntFieldUpdateOperationsInput | number
    pack_id?: NullableIntFieldUpdateOperationsInput | number | null
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type TransactionsUpdateWithoutUserInput = {
    pid?: StringFieldUpdateOperationsInput | string
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    currency?: StringFieldUpdateOperationsInput | string
    payment_id?: StringFieldUpdateOperationsInput | string
    order_id?: StringFieldUpdateOperationsInput | string
    plan?: StringFieldUpdateOperationsInput | string
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type TransactionsUncheckedUpdateWithoutUserInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    currency?: StringFieldUpdateOperationsInput | string
    payment_id?: StringFieldUpdateOperationsInput | string
    order_id?: StringFieldUpdateOperationsInput | string
    plan?: StringFieldUpdateOperationsInput | string
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type TransactionsUncheckedUpdateManyWithoutUserInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    credit_amount?: IntFieldUpdateOperationsInput | number
    model_amount?: IntFieldUpdateOperationsInput | number
    currency?: StringFieldUpdateOperationsInput | string
    payment_id?: StringFieldUpdateOperationsInput | string
    order_id?: StringFieldUpdateOperationsInput | string
    plan?: StringFieldUpdateOperationsInput | string
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type ImagesCreateManyTraining_modelInput = {
    id?: number
    pid: string
    user_id: number
    pack_id?: number | null
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type ImagesUpdateWithoutTraining_modelInput = {
    pid?: StringFieldUpdateOperationsInput | string
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    user?: UsersUpdateOneRequiredWithoutImagesNestedInput
    pack?: PacksUpdateOneWithoutImagesNestedInput
  }

  export type ImagesUncheckedUpdateWithoutTraining_modelInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    pack_id?: NullableIntFieldUpdateOperationsInput | number | null
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type ImagesUncheckedUpdateManyWithoutTraining_modelInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    pack_id?: NullableIntFieldUpdateOperationsInput | number | null
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type ImagesCreateManyPackInput = {
    id?: number
    pid: string
    user_id: number
    training_model_id: number
    user_prompt: string
    sys_prompt: string
    alt: string
    num_inference_steps: number
    content_type: $Enums.ImageFormat
    status: $Enums.Status
    image_size: $Enums.ImageSize
    fal_ai_request_id?: string | null
    width?: number | null
    height?: number | null
    image_url?: string | null
    image_url_s3?: string | null
    is_favorite?: boolean
    deleted_at?: Date | string | null
    created_at?: Date | string
    updated_at?: Date | string
  }

  export type ImagesUpdateWithoutPackInput = {
    pid?: StringFieldUpdateOperationsInput | string
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
    user?: UsersUpdateOneRequiredWithoutImagesNestedInput
    training_model?: TrainingModelsUpdateOneRequiredWithoutImagesNestedInput
  }

  export type ImagesUncheckedUpdateWithoutPackInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    training_model_id?: IntFieldUpdateOperationsInput | number
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }

  export type ImagesUncheckedUpdateManyWithoutPackInput = {
    id?: IntFieldUpdateOperationsInput | number
    pid?: StringFieldUpdateOperationsInput | string
    user_id?: IntFieldUpdateOperationsInput | number
    training_model_id?: IntFieldUpdateOperationsInput | number
    user_prompt?: StringFieldUpdateOperationsInput | string
    sys_prompt?: StringFieldUpdateOperationsInput | string
    alt?: StringFieldUpdateOperationsInput | string
    num_inference_steps?: IntFieldUpdateOperationsInput | number
    content_type?: EnumImageFormatFieldUpdateOperationsInput | $Enums.ImageFormat
    status?: EnumStatusFieldUpdateOperationsInput | $Enums.Status
    image_size?: EnumImageSizeFieldUpdateOperationsInput | $Enums.ImageSize
    fal_ai_request_id?: NullableStringFieldUpdateOperationsInput | string | null
    width?: NullableIntFieldUpdateOperationsInput | number | null
    height?: NullableIntFieldUpdateOperationsInput | number | null
    image_url?: NullableStringFieldUpdateOperationsInput | string | null
    image_url_s3?: NullableStringFieldUpdateOperationsInput | string | null
    is_favorite?: BoolFieldUpdateOperationsInput | boolean
    deleted_at?: NullableDateTimeFieldUpdateOperationsInput | Date | string | null
    created_at?: DateTimeFieldUpdateOperationsInput | Date | string
    updated_at?: DateTimeFieldUpdateOperationsInput | Date | string
  }



  /**
   * Batch Payload for updateMany & deleteMany & createMany
   */

  export type BatchPayload = {
    count: number
  }

  /**
   * DMMF
   */
  export const dmmf: runtime.BaseDMMF
}