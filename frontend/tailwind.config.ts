import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: ["class"],
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./lib/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-conic":
          "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
      },
      backgroundColor: {
        "main-translucent": "rgba(255, 255, 255, 0.3)", // 半透明の白色背景
        "background-main": "#f4ede8",
        "background-dialog": "#ffffff",
      },
      colors: {
        primary: "#1F2937",
        secondary: "#F3F4F6",
        accent: {
          DEFAULT: "#EC4899", // 既存のアクセントカラー
          dark: "#bc397a", // より暗いアクセントカラー
        },
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        text: {
          primary: "#1F2937",
          secondary: "#6B7280",
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
        },
        navigation: {
          bg: "#1F2937", // ナビゲーションバーの背景色
        },
        dialog: {
          bg: "#ffffff", // ダイアログの背景色
          header: "#ffffff", // ヘッダーの背景色
          hover: "#f9f6f3", // ホバー時の背景色
          selected: "#f9f6f3", // 選択された項目の背景色
        },
        main: {
          bg: "#EBDFD7", // メイン画面の背景色
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        chart: {
          "1": "hsl(var(--chart-1))",
          "2": "hsl(var(--chart-2))",
          "3": "hsl(var(--chart-3))",
          "4": "hsl(var(--chart-4))",
          "5": "hsl(var(--chart-5))",
        },
        toast: {
          success: {
            background: "#E8F5E9", // より柔らかい緑色
            text: "#2E7D32", // 濃い緑のテキスト
          },
          error: {
            background: "#FFEBEE", // より柔らかい赤色
            text: "#C62828", // 濃い赤のテキスト
          },
          info: {
            background: "#E3F2FD", // より柔らかい青色
            text: "#1565C0", // 濃い青のテキスト
          },
          warning: {
            background: "#FFF3E0", // より柔らかいオレンジ色
            text: "#EF6C00", // 濃いオレンジのテキスト
          },
        },
      },
      fontFamily: {
        sans: [
          "ui-sans-serif",
          "system-ui",
          "-apple-system",
          "BlinkMacSystemFont",
          '"Segoe UI"',
          "Roboto",
          '"Helvetica Neue"',
          "Arial",
          '"Noto Sans"',
          "sans-serif",
          '"Apple Color Emoji"',
          '"Segoe UI Emoji"',
          '"Segoe UI Symbol"',
          '"Noto Color Emoji"',
        ],
        serif: [
          "ui-serif",
          "Georgia",
          "Cambria",
          '"Times New Roman"',
          "Times",
          "serif",
        ],
        mono: [
          "ui-monospace",
          "SFMono-Regular",
          "Menlo",
          "Monaco",
          "Consolas",
          '"Liberation Mono"',
          '"Courier New"',
          "monospace",
        ],
      },
      fontSize: {
        xs: "0.75rem",
        sm: "0.875rem",
        base: "1rem",
        lg: "1.125rem",
        xl: "1.25rem",
        "2xl": "1.5rem",
      },
      fontWeight: {
        normal: "400",
        medium: "500",
        semibold: "600",
        bold: "700",
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
    },
  },
  plugins: [require("tailwindcss-animate")],
};

export default config;
