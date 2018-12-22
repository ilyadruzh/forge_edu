# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :elm_articles,
  ecto_repos: [ElmArticles.Repo]

# Configures the endpoint
config :elm_articles, ElmArticlesWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "ndW1fgCc2jlCWQ5GEtjQaMUu1E/PI2DfUYr007U5UxI2xlLM6WfwVFeLenuASoEm",
  render_errors: [view: ElmArticlesWeb.ErrorView, accepts: ~w(html json)],
  pubsub: [name: ElmArticles.PubSub, adapter: Phoenix.PubSub.PG2]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
