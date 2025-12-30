// import "@typespec/http";
// import "@typespec/rest";
// import "@typespec/openapi";
// import "@typespec/openapi3";

// using Http;
// using Rest;
// using TypeSpec.OpenAPI;

// @service(#{ title: "DDNS Service" })
// @info(#{ version: "0.1.0" })
// @doc("This is the API specification for the Dynamic DNS Service (DDNS). It allows users to authenticate, manage DNS records, and update their IP addresses dynamically.")
// @server("https://ddns.duacodie.com", "Production")
// namespace Ddns;

// enum IpFamily {
//   v4,
//   v6,
// }

// model ErrorResponse {
//   code: string;
//   message: string;
//   requestId?: string;
// }

// model TokenPair {
//   accessToken: string;
//   refreshToken: string;
//   tokenType: "Bearer";
//   expiresInSec: int32;
// }

// model LoginRequest {
//   username: string;
//   password: string;
//   deviceName?: string;
// }

// model RefreshRequest {
//   refreshToken: string;
// }

// model LogoutRequest {
//   refreshToken: string;
// }

// model RecordInfo {
//   recordId: string;
//   zoneId: string;
//   name: string;
//   type: "A" | "AAAA";
//   proxied?: boolean;
//   ttl?: int32;
// }

// model ListRecordsResponse {
//   records: RecordInfo[];
// }

// model UpdateRecordRequest {
//   recordId: string;
//   ip: string;
//   family: IpFamily;
//   detectedBy?: "ifconfig" | "stun" | "http" | "cloudflare-trace";
// }

// model UpdateRecordResponse {
//   recordId: string;
//   name: string;
//   type: "A" | "AAAA";
//   oldContent?: string;
//   newContent: string;
//   updatedAt: utcDateTime;
// }

// interface Auth {
//   @route("/auth/login")
//   @post
//   login(@body body: LoginRequest): TokenPair | ErrorResponse;

//   @route("/auth/refresh")
//   @post
//   refresh(@body body: RefreshRequest): TokenPair | ErrorResponse;

//   @route("/auth/logout")
//   @post
//   logout(@body body: LogoutRequest): {} | ErrorResponse;
// }

// @useAuth(BearerAuth)
// interface Records {
//   @route("/records")
//   @get
//   list(): ListRecordsResponse | ErrorResponse;
// }

// @useAuth(BearerAuth)
// interface Updates {
//   @route("/records:update")
//   @post
//   update(@body body: UpdateRecordRequest): UpdateRecordResponse | ErrorResponse;
// }


import "@typespec/http";
import "@typespec/rest";
import "@typespec/openapi";
import "@typespec/openapi3";

using Http;
using Rest;
using TypeSpec.OpenAPI;

@service(#{ title: "DDNS Service" })
@info(#{ version: "0.1.0" })
@server("https://ddns.duacodie.com", "Production")
namespace Ddns;

@useAuth(BearerAuth)
interface _AuthProtected {}

// ---- Security Scheme ----

// ---- Common ----
enum ErrorCode {
  INVALID_CREDENTIALS,
  TOKEN_EXPIRED,
  TOKEN_REVOKED,
  UNAUTHORIZED,
  FORBIDDEN,
  NOT_FOUND,
  VALIDATION_ERROR,
  RATE_LIMITED,
  INTERNAL_ERROR,
}

model ErrorResponse {
  code: ErrorCode;
  message: string;
  requestId?: string;
}

// TokenPair: access token is JWT; refresh token is opaque string
model TokenPair {
  accessToken: string;
  refreshToken: string;
  tokenType: "Bearer";
  expiresInSec: int32;
}

model LoginRequest {
  username: string;
  password: string;
  deviceName?: string;
}

model RefreshRequest {
  refreshToken: string;
}

model LogoutRequest {
  refreshToken: string;
}

// ---- Records ----
model RecordInfo {
  recordId: string;   // your internal id (UUID), NOT cloudflare id
  zoneId: string;     // cloudflare zone id
  name: string;       // e.g. "home.example.com"
  type: "A" | "AAAA";
  proxied?: boolean;
  ttl?: int32;
}

model ListRecordsResponse {
  records: RecordInfo[];
}

model UpdateRecordRequest {
  // recordId moved to path; avoid mismatch
  ip: string; // server validates v4/v6 and matches record type
  detectedBy?: "ifconfig" | "stun" | "http" | "cloudflare-trace";
}

model UpdateRecordResponse {
  recordId: string;
  name: string;
  type: "A" | "AAAA";
  oldContent?: string;
  newContent: string;
  updatedAt: utcDateTime;
}

// ---- Interfaces ----
interface Auth {
  @route("/auth/login")
  @post
  login(@body body: LoginRequest): TokenPair | ErrorResponse;

  @route("/auth/refresh")
  @post
  refresh(@body body: RefreshRequest): TokenPair | ErrorResponse;

  @route("/auth/logout")
  @post
  logout(@body body: LogoutRequest): {} | ErrorResponse;
}

interface Records extends _AuthProtected {
  @route("/records")
  @get
  list(): ListRecordsResponse | ErrorResponse;
}

interface Updates extends _AuthProtected {
  @route("/records/{recordId}:update")
  @post
  update(@path recordId: string, @body body: UpdateRecordRequest): UpdateRecordResponse | ErrorResponse;
}
