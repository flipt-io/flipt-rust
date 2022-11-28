use flipt::api::{
    constraint::{ComparisonType, Constraint, ConstraintCreateRequest, Operator},
    distribution::DistributionCreateRequest,
    evaluation::{EvaluateRequest, Reason},
    flag::FlagCreateRequest,
    rule::{Rule, RuleCreateRequest},
    segment::{Match, SegmentCreateRequest},
    variant::{Variant, VariantCreateRequest},
    ApiClient,
};
use flipt::auth::{token::TokenCreateRequest, token::TokenListRequest, AuthClient};
use flipt::Config;

#[tokio::test]
#[cfg_attr(not(feature = "flipt_integration"), ignore)]
async fn integration_api() {
    let config = Config::new_from_env().expect("config");
    let client = ApiClient::new(config).expect("build client");

    let _ = client.flags().delete("flag-a").await;
    let _ = client.variants().delete("flag-a", "variant-a").await;
    let _ = client.segments().delete("segment-a").await;

    create_flag(&client, "flag-a").await;
    let variant = create_variant(&client, "flag-a", "variant-a").await;
    create_segment(&client, "segment-a").await;
    let constraint = create_constraint(&client, "segment-a").await;
    let rule = create_rule(&client, "flag-a", "segment-a").await;
    create_distribution(&client, "flag-a", &rule.id, &variant.id).await;
    evaluate(&client, "flag-a").await;

    let _ = client.flags().delete("flag-a").await;
    let _ = client.variants().delete("flag-a", "variant-a").await;
    let _ = client.segments().delete("segment-a").await;
    let _ = client
        .constraints()
        .delete("segment-a", &constraint.id)
        .await;
    let _ = client.rules().delete("flag-a", &rule.id).await;

    async fn create_flag(client: &ApiClient, key: &str) {
        let flag = client
            .flags()
            .create(&FlagCreateRequest {
                key: key.into(),
                name: "Flag".into(),
                enabled: true,
                ..Default::default()
            })
            .await
            .expect("create flag");

        assert_eq!(flag.key, key);
        assert_eq!(flag.name, "Flag");
        assert_eq!(flag.description, "");
        assert!(flag.enabled);
    }

    async fn create_variant(client: &ApiClient, flag_key: &str, key: &str) -> Variant {
        let variant = client
            .variants()
            .create(
                flag_key,
                &VariantCreateRequest {
                    key: key.into(),
                    name: "Variant".into(),
                    ..Default::default()
                },
            )
            .await
            .expect("create variant");

        assert_eq!(variant.key, key);
        assert_eq!(variant.name, "Variant");
        assert_eq!(variant.description, "");

        variant
    }

    async fn create_segment(client: &ApiClient, key: &str) {
        let segment = client
            .segments()
            .create(&SegmentCreateRequest {
                key: key.into(),
                name: "Segment".into(),
                ..Default::default()
            })
            .await
            .expect("create variant");
        assert_eq!(segment.key, key);
        assert_eq!(segment.name, "Segment");
        assert_eq!(segment.description, "");
        assert_eq!(segment.match_type, Match::Any);
    }

    async fn create_constraint(client: &ApiClient, segment: &str) -> Constraint {
        let constraint = client
            .constraints()
            .create(
                segment,
                &ConstraintCreateRequest {
                    property: "name".into(),
                    value: "brett".into(),
                    operator: Operator::Eq,
                    comparison_type: ComparisonType::String,
                },
            )
            .await
            .expect("create segment");

        assert!(constraint.id.len() != 0);
        assert_eq!(constraint.operator, Operator::Eq);
        assert_eq!(constraint.property, "name");
        assert_eq!(constraint.value, "brett");
        assert_eq!(constraint.comparison_type, ComparisonType::String);

        constraint
    }

    async fn create_rule(client: &ApiClient, flag_key: &str, segment_key: &str) -> Rule {
        let rule = client
            .rules()
            .create(
                flag_key,
                &RuleCreateRequest {
                    rank: 1,
                    segment_key: segment_key.into(),
                },
            )
            .await
            .expect("create rule");

        assert!(rule.id.len() != 0);
        assert_eq!(rule.flag_key, flag_key);
        assert_eq!(rule.rank, 1u32);
        assert_eq!(rule.segment_key, "segment-a");
        assert_eq!(rule.distributions, (&[]).to_vec());

        rule
    }

    async fn create_distribution(
        client: &ApiClient,
        flag_key: &str,
        rule_id: &str,
        variant_id: &str,
    ) {
        let dist = client
            .distributions()
            .create(
                flag_key,
                rule_id,
                &DistributionCreateRequest {
                    rollout: 100.0,
                    variant_id: variant_id.into(),
                },
            )
            .await
            .expect("create distribution");

        assert!(dist.id.len() != 0);
        assert_eq!(dist.rule_id, rule_id);
        assert_eq!(dist.variant_id, variant_id);
        assert_eq!(dist.rollout, 100.0);
    }

    async fn evaluate(client: &ApiClient, flag_key: &str) {
        let eval = client
            .evaluation()
            .evaluate(&EvaluateRequest {
                entity_id: "abc".into(),
                context: std::collections::HashMap::from([(
                    String::from("name"),
                    String::from("brett"),
                )]),
                flag_key: flag_key.into(),
                ..Default::default()
            })
            .await
            .expect("eval");
        assert!(eval.is_match);
        assert_eq!(eval.reason, Reason::Match);
    }
}

#[tokio::test]
#[cfg_attr(not(feature = "flipt_integration"), ignore)]
async fn integration_auth() {
    let config = Config::new_from_env().expect("config");
    let client = AuthClient::new(config).expect("build client");

    let list = client
        .tokens()
        .list(&TokenListRequest::default())
        .await
        .expect("list tokens");

    for auth in list.authentications {
        if auth.metadata[flipt::auth::METADATA_LABEL_NAME] == "e2e" {
            let _ = client.tokens().delete(&auth.id).await;
        }
    }

    let token = client
        .tokens()
        .create(&TokenCreateRequest {
            name: "e2e".into(),
            description: "foobar".into(),
            ..Default::default()
        })
        .await
        .expect("create token");
    assert_ne!(token.authentication.id, "");
    assert_eq!(token.authentication.expires_at, None);

    let metadata = token.authentication.metadata;
    assert_eq!(
        metadata
            .get(&String::from(flipt::auth::METADATA_LABEL_NAME))
            .expect("name from metadata"),
        "e2e"
    );
    assert_eq!(
        metadata
            .get(&String::from(flipt::auth::METADATA_LABEL_DESCRIPTION))
            .expect("description from metadata"),
        "foobar"
    );

    let _ = client.tokens().delete(&token.authentication.id).await;

    let me = client.me().await.expect("me");
    assert_ne!(me.id, "");
}
