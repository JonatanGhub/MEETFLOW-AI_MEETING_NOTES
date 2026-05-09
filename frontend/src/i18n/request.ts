// next-intl server config — used only when running with Next.js server.
// In Tauri (static export) the locale is resolved client-side via LocaleProvider.
import { getRequestConfig } from "next-intl/server";
import { defaultLocale } from "./config";

export default getRequestConfig(async () => {
  // Tauri static export: always default to 'en' server-side.
  // Actual locale switching is handled by the client LocaleProvider.
  const locale = defaultLocale;
  return {
    locale,
    messages: (await import(`../messages/${locale}.json`)).default,
  };
});
