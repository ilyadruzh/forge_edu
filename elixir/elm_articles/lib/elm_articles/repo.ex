defmodule ElmArticles.Repo do
  use Ecto.Repo,
    otp_app: :elm_articles,
    adapter: Ecto.Adapters.Postgres
end
