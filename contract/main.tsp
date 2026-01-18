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

model RecordInfo {
  recordId: string;   // id (UUID), NOT cloudflare id
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

@route("/v1")
namespace Ddns.V1 {
  @route("/auth")
  interface Auth {
    @route("/login")
    @post
    login(@body body: LoginRequest): TokenPair | ErrorResponse;

    @route("/refresh")
    @post
    refresh(@body body: RefreshRequest): TokenPair | ErrorResponse;

    @route("/logout")
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
}
