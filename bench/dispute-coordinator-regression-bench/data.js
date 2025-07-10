window.BENCHMARK_DATA = {
  "lastUpdate": 1752133225137,
  "repoUrl": "https://github.com/paritytech/polkadot-sdk",
  "entries": {
    "dispute-coordinator-regression-bench": [
      {
        "commit": {
          "author": {
            "email": "eresav@me.com",
            "name": "Andrei Eres",
            "username": "AndreiEres"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "83b0409093f811acb412b07ac7219b7ad1a514ff",
          "message": "[subsystem-bench] Add Dispute Coordinator subsystem benchmark (#8828)\n\nFixes https://github.com/paritytech/polkadot-sdk/issues/8811\n\n---------\n\nCo-authored-by: cmd[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-03T12:22:23Z",
          "tree_id": "7dedca9f4f5317f038bb7713852df1f21eeee806",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/83b0409093f811acb412b07ac7219b7ad1a514ff"
        },
        "date": 1751549436117,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "test-environment",
            "value": 0.005595405729999999,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008679936599999995,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.0026281824699999996,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "egor@parity.io",
            "name": "Egor_P",
            "username": "EgorPopelyaev"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "3bd01b9c89dbef0f57a3c0fb7f600fbb5befff65",
          "message": "[Release|CI/CD] Fix syncing in the release flow (#9092)\n\nThis PR adds a fix for the release pipelines. The sync flow needs a\nsecrete to be passed when it is called from another flow and syncing\nbetween release org and the main repo is needed.\nMissing secrets were added to the appropriate flows.",
          "timestamp": "2025-07-03T15:06:37Z",
          "tree_id": "806f5adc03322aa929b1b29440cb9212f69c9fe8",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/3bd01b9c89dbef0f57a3c0fb7f600fbb5befff65"
        },
        "date": 1751559377721,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "test-environment",
            "value": 0.005582663829999996,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.0026697256099999993,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008752567599999988,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "10196091+Ank4n@users.noreply.github.com",
            "name": "Ankan",
            "username": "Ank4n"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "f1ba2a1c7206c70ad66168859c90ab4e4327aab6",
          "message": "Optimize buffered offence storage and prevent unbounded growth in staking-async ah-client pallet (#9049)\n\n## 🤔 Why\nThis addresses potential memory issues and improves efficiency of\noffence handling during buffered operating mode (see\nhttps://github.com/paritytech-secops/srlabs_findings/issues/525)\n\n\n## 🔑 Key changes\n\n- Prevents duplicate offences for the same offender in the same session\nby keeping only the highest slash fraction\n- Introduces `BufferedOffence` struct with optional reporter and slash\nfraction fields\n- Restructures buffered offences storage from `Vec<(SessionIndex,\nVec<Offence>)>` to nested `BTreeMap<SessionIndex, BTreeMap<AccountId,\nBufferedOffence>>`\n- Adds `MaxOffenceBatchSize` configuration parameter for batching\ncontrol\n- Processes offences in batches with configurable size limits, sending\nonly first session's offences per block\n- Implements proper benchmarking infrastructure for\n`process_buffered_offences` function\n- Adds WeightInfo trait with benchmarked weights for batch processing in\n`on_initialize` hook\n\n## ✍️ Co-authors\n@Ank4n \n@sigurpol\n\n---------\n\nCo-authored-by: Paolo La Camera <paolo@parity.io>\nCo-authored-by: cmd[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-04T09:02:33Z",
          "tree_id": "410487862394418dd87119db2954a36e4de0c43c",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/f1ba2a1c7206c70ad66168859c90ab4e4327aab6"
        },
        "date": 1751623985007,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.002641694280000002,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.00871780210999999,
            "unit": "seconds"
          },
          {
            "name": "test-environment",
            "value": 0.005657479960000001,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "oliver.tale-yazdi@parity.io",
            "name": "Oliver Tale-Yazdi",
            "username": "ggwpez"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "22714211e4f558abbabae28fc2e8f2c971143638",
          "message": "[AHM] Derive DecodeWithMemTracking and pub fields (#9067)\n\n- Derive `DecodeWithMemTracking` on structs\n- Make some fields public\n\n---------\n\nSigned-off-by: Oliver Tale-Yazdi <oliver.tale-yazdi@parity.io>",
          "timestamp": "2025-07-04T10:36:12Z",
          "tree_id": "0dd0655d92d837e407ee908f523b783ecccc626a",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/22714211e4f558abbabae28fc2e8f2c971143638"
        },
        "date": 1751629886195,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "test-environment",
            "value": 0.005486065759999997,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008570165919999994,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.00267932138,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "5588131+kianenigma@users.noreply.github.com",
            "name": "Kian Paimani",
            "username": "kianenigma"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "252649fc0105efc8b32b2e1a3649bd6d09f8bd53",
          "message": "add benchmark for prune-era (#9056)\n\nCo-authored-by: cmd[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-04T18:25:54Z",
          "tree_id": "c4480f0f14cd79f70f4a2733fab6a6d0c4c81f6b",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/252649fc0105efc8b32b2e1a3649bd6d09f8bd53"
        },
        "date": 1751657691195,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "test-environment",
            "value": 0.005649167919999998,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008880581469999996,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.0026971257299999987,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "james@jsdw.me",
            "name": "James Wilson",
            "username": "jsdw"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "771c9988e2a636a150d97c10e3122af8068d1687",
          "message": "Bump CI to Rustc 1.88 to support 2024 edition crates (#8592)\n\nAs one example, this allows us to use the latest version of Subxt: 0.42.\nAlso if-let chains :)\n\nMain changes:\n- Update CI image\n- Remove `forklift` from Build step in\n`check-revive-stable-uapi-polkavm`; it seemed to [cause an\nerror](https://github.com/paritytech/polkadot-sdk/actions/runs/16004536662/job/45148002314?pr=8592).\nPerhaps we can open an issue for this to fix/try again after this\nmerges.\n- Bump `polkavm` deps to 0.26 to avoid [this\nerror](https://github.com/paritytech/polkadot-sdk/actions/runs/16004991577/job/45150325849?pr=8592#step:5:1967)\n(thanks @koute!)\n- Add `result_large_err` clippy to avoid a bunch of clippy warnings\nabout a 176 byte error (again, we could fix this later more properly).\n- Clippy fixes (mainly inlining args into `format!`s where possible),\nremove one `#[no_mangle]` on a `#[panic_hook]` and a few other misc\nautomatic fixes.\n- `#[allow(clippy::useless_conversion)]` in frame macro to avoid the\ngenerated `.map(Into::into).map_err(Into::into)` code causing an issue\nwhen not necessary (it is sometimes; depends on the return type in\npallet calls)\n- UI test updates\n\nAs a side note, I haven't added a `prdoc` since I'm not making any\nbreaking changes (despite touching a bunch of pallets), just clippy/fmt\ntype things. Please comment if this isn't ok!\n\nAlso, thankyou @bkchr for the wasmtime update PR which fixed a blocker\nhere!\n\n---------\n\nCo-authored-by: Evgeny Snitko <evgeny@parity.io>\nCo-authored-by: Bastian Köcher <git@kchr.de>",
          "timestamp": "2025-07-04T21:54:27Z",
          "tree_id": "bbce6a530538cfc5d3328f5239b16d133890b86d",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/771c9988e2a636a150d97c10e3122af8068d1687"
        },
        "date": 1751670346956,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008583395449999991,
            "unit": "seconds"
          },
          {
            "name": "test-environment",
            "value": 0.0051193470899999925,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.0025815619300000006,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "14218860+iulianbarbu@users.noreply.github.com",
            "name": "Iulian Barbu",
            "username": "iulianbarbu"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "436b4935b52562f79a83b6ecadeac7dcbc1c2367",
          "message": "`polkadot-omni-node`: pass timestamp inherent data for block import (#9102)\n\n# Description\n\nThis should allow aura runtimes to check timestamp inherent data to\nsync/import blocks that include timestamp inherent data.\n\nCloses #8907 \n\n## Integration\n\nRuntime developers can check timestamp inherent data while using\n`polkadot-omni-node-lib`/`polkadot-omni-node`/`polkadot-parachain`\nbinaries. This change is backwards compatible and doesn't require\nruntimes to check the timestamp inherent, but they are able to do it now\nif needed.\n\n## Review Notes\n\nN/A\n\n---------\n\nSigned-off-by: Iulian Barbu <iulian.barbu@parity.io>\nCo-authored-by: cmd[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-06T09:32:11Z",
          "tree_id": "239ba865d190c48c06af7d1fa35ceb411cc31cea",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/436b4935b52562f79a83b6ecadeac7dcbc1c2367"
        },
        "date": 1751798589854,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "dispute-distribution",
            "value": 0.00855703834999999,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.002733640860000001,
            "unit": "seconds"
          },
          {
            "name": "test-environment",
            "value": 0.005003881119999989,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "git@kchr.de",
            "name": "Bastian Köcher",
            "username": "bkchr"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "cb12563ae4e532876c29b67be9a7f5d06fdc9fc3",
          "message": "Replace `assert_para_throughput` with `assert_finalized_para_throughput` (#9117)\n\nThere is no need to have two functions which are essentially doing the\nsame. It is also better to oberserve the finalized blocks, which also\nsimplifies the code. So, this pull request is replacing the\n`assert_para_throughput` with `assert_finalized_para_throughput`. It\nalso replaces any usage of `assert_finalized_para_throughput` with\n`assert_para_throughput`.\n\n---------\n\nCo-authored-by: cmd[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-08T16:04:23Z",
          "tree_id": "faed545176a9de8b004b29e5ee7e4b5c2ccecef6",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/cb12563ae4e532876c29b67be9a7f5d06fdc9fc3"
        },
        "date": 1751995024154,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.0026695474100000005,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.00859867911999999,
            "unit": "seconds"
          },
          {
            "name": "test-environment",
            "value": 0.005122107889999993,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49718502+alexggh@users.noreply.github.com",
            "name": "Alexandru Gheorghe",
            "username": "alexggh"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "88fc41c9cf5e46277b7cab53a72c650b75377d25",
          "message": "make 0002-parachains-disputes a bit more robust (#9074)\n\nThere is inherently a race between the time we snapshot\nfinality_lag/disputes_finality_lag metrics and if the dispute/approvals\nfinished, so sometimes the test was failing because it was reporting 1\nwhich is in no way a problem, so let's make it a bit more robust by\nsimply waiting more time to reach 0.\n\nFixes: https://github.com/paritytech/polkadot-sdk/issues/8941.\n\n---------\n\nSigned-off-by: Alexandru Gheorghe <alexandru.gheorghe@parity.io>",
          "timestamp": "2025-07-08T16:10:51Z",
          "tree_id": "8a90317b0febd3a60f76b56d7a854edcf7a4085d",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/88fc41c9cf5e46277b7cab53a72c650b75377d25"
        },
        "date": 1751997109460,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.0026244691599999993,
            "unit": "seconds"
          },
          {
            "name": "test-environment",
            "value": 0.005114807139999997,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008560092539999998,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "franciscoaguirreperez@gmail.com",
            "name": "Francisco Aguirre",
            "username": "franciscoaguirre"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4d5e95217831fb75942d8153a22f6864858c1d71",
          "message": "XCM precompile: don't support older xcm versions (#9126)\n\nThe latest XCM version is 5. A lot of parachains are still running V3 or\nV4 which is why we haven't removed them, but the XCM precompile is new\nand should only have to deal with versions 5 and onwards. No need to\nkeep dragging 3 and 4 in contracts.",
          "timestamp": "2025-07-08T17:27:43Z",
          "tree_id": "2944a79e52968a0f54da0a246a07867b8f95dffe",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/4d5e95217831fb75942d8153a22f6864858c1d71"
        },
        "date": 1752000039848,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "test-environment",
            "value": 0.005085985199999996,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.00263165981,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.00852913286999999,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "pgherveou@gmail.com",
            "name": "PG Herveou",
            "username": "pgherveou"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "9b7c20a2a187e57433c055592609e35af0258bbc",
          "message": "Fix seal_call benchmark (#9112)\n\nFix seal_call benchmark, ensure that the benchmarked block actually\nsucceed\n\n---------\n\nCo-authored-by: cmd[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-08T18:30:43Z",
          "tree_id": "a5d64f5c7d1bffccf857ee5ff83a6f6b305f5ee0",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/9b7c20a2a187e57433c055592609e35af0258bbc"
        },
        "date": 1752004430350,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.0026429404299999986,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008568671429999996,
            "unit": "seconds"
          },
          {
            "name": "test-environment",
            "value": 0.005159262569999995,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "franciscoaguirreperez@gmail.com",
            "name": "Francisco Aguirre",
            "username": "franciscoaguirre"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ba2a8dc536db30397c332a2aa2cd9f9863027093",
          "message": "XCM precompile: small cleanup (#9135)\n\nFollow-up to\nhttps://github.com/paritytech/polkadot-sdk/pull/9125#discussion_r2192896809",
          "timestamp": "2025-07-08T19:47:45Z",
          "tree_id": "e7aeb64bf7cbd7d415bc142f30193c7d6ec3f579",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/ba2a8dc536db30397c332a2aa2cd9f9863027093"
        },
        "date": 1752008673216,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "test-environment",
            "value": 0.00520881492999999,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.0026551542999999995,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008714952019999993,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dharjeezy@gmail.com",
            "name": "dharjeezy",
            "username": "dharjeezy"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cc972542e0df0266cde2ead4cfac3b1558c860af",
          "message": "pallet bounties v2 benchmark (#8952)\n\ncloses #8649\n\n---------\n\nCo-authored-by: cmd[bot] <41898282+github-actions[bot]@users.noreply.github.com>\nCo-authored-by: Bastian Köcher <git@kchr.de>",
          "timestamp": "2025-07-08T21:47:29Z",
          "tree_id": "92ea303bb8df02e5752f9903f5541e35918ac3a9",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/cc972542e0df0266cde2ead4cfac3b1558c860af"
        },
        "date": 1752015675272,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.0026522110800000004,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008721413299999987,
            "unit": "seconds"
          },
          {
            "name": "test-environment",
            "value": 0.005168960659999988,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "Sajjon@users.noreply.github.com",
            "name": "Alexander Cyon",
            "username": "Sajjon"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7ab0dcd62887ea3c5e50cfb5b1b01beb09d0ec92",
          "message": "Add `para_ids` Runtime API (#9055)\n\nImplementation of https://github.com/paritytech/polkadot-sdk/issues/9053\n\n---------\n\nCo-authored-by: alindima <alin@parity.io>",
          "timestamp": "2025-07-09T07:17:25Z",
          "tree_id": "efefbe78f8e545dae503496bbc822b03e32d1e13",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/7ab0dcd62887ea3c5e50cfb5b1b01beb09d0ec92"
        },
        "date": 1752049594274,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.002608908810000001,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008476387969999994,
            "unit": "seconds"
          },
          {
            "name": "test-environment",
            "value": 0.005002263799999994,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "egor@parity.io",
            "name": "Egor_P",
            "username": "EgorPopelyaev"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "cd39c26a4da04693b07b3ed752ea239f452795ea",
          "message": "[Release|CI/CD] Move runtimes build to a separate pipeline and let it trigger the next flow (#9118)\n\nThis PR incudes the following changes:\n- Cut the runtimes build from the Create Draft flow into a standalone\npipeline\n- Add a trigger to the Build Runtimes pipeline that will be starting the\nCreate Draft flow automatically when the runtimes are built\nsuccessfully.\n\nCloses: https://github.com/paritytech/devops/issues/3827 and partially:\nhttps://github.com/paritytech/devops/issues/3828",
          "timestamp": "2025-07-09T08:40:25Z",
          "tree_id": "69aff4dc6192fec945b7a0b030222c92ac453a33",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/cd39c26a4da04693b07b3ed752ea239f452795ea"
        },
        "date": 1752054592670,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.00271226005,
            "unit": "seconds"
          },
          {
            "name": "test-environment",
            "value": 0.005194933789999989,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008831572839999986,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "bkontur@gmail.com",
            "name": "Branislav Kontur",
            "username": "bkontur"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "83afbeeb906131755fdcea3b891ea1883c4d17d0",
          "message": "Expose more constants for pallet-xcm (#9139)\n\nLet's expose more constants, similar as `AdvertisedXcmVersion`.\n\n\n![image](https://github.com/user-attachments/assets/5ddc265f-546b-45a0-8235-3f53c3108823)\n\n---------\n\nCo-authored-by: cmd[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-09T12:29:35Z",
          "tree_id": "6fb2c4c504887609989d96ab44ba1a1afbe03294",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/83afbeeb906131755fdcea3b891ea1883c4d17d0"
        },
        "date": 1752068758017,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "test-environment",
            "value": 0.005127152969999997,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.00260139055,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.00855473461999999,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "117115317+lrubasze@users.noreply.github.com",
            "name": "Lukasz Rubaszewski",
            "username": "lrubasze"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7305f96aa8fc68b7249587c21f5fa2d4520c54cd",
          "message": "CI - zombienet cumulus tests zombienet sdk (#8954)\n\n### This PR includes the following changes:\n\n- Migrates Zombienet Cumulus tests to `zombienet-sdk`\n- Re-enables the tests, with the following exceptions (to be addressed\nseparately):\n  - `zombienet-cumulus-0002-pov_recovery` - #8985 \n- `zombienet-cumulus-0006-rpc_collator_builds_blocks` - root cause the\nsame as #8985\n  - `zombienet-cumulus-0009-elastic_scaling_pov_recovery` – #8999\n- `zombienet-cumulus-0010-elastic_scaling_multiple_block_per_slot` –\n#9018\n- Adds the following tests to CI:\n  - `zombienet-cumulus-0011-dht-bootnodes`\n  - `zombienet-cumulus-0012-parachain_extrinsic_gets_finalized`\n  - `zombienet-cumulus-0013-elastic_scaling_slot_based_rp_offset`\n\n---------\n\nSigned-off-by: Iulian Barbu <iulian.barbu@parity.io>\nCo-authored-by: Javier Viola <javier@parity.io>\nCo-authored-by: Javier Viola <363911+pepoviola@users.noreply.github.com>\nCo-authored-by: cmd[bot] <41898282+github-actions[bot]@users.noreply.github.com>\nCo-authored-by: Anthony Lazam <xlzm.tech@gmail.com>\nCo-authored-by: Sebastian Kunert <skunert49@gmail.com>\nCo-authored-by: Iulian Barbu <14218860+iulianbarbu@users.noreply.github.com>\nCo-authored-by: Bastian Köcher <info@kchr.de>",
          "timestamp": "2025-07-09T16:01:41Z",
          "tree_id": "7b46e0ac8c2ed95e791c472fb7a82ebbc6a32685",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/7305f96aa8fc68b7249587c21f5fa2d4520c54cd"
        },
        "date": 1752081449064,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "test-environment",
            "value": 0.004915220189999994,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.00252144691,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008389578499999993,
            "unit": "seconds"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "117115317+lrubasze@users.noreply.github.com",
            "name": "Lukasz Rubaszewski",
            "username": "lrubasze"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "409587adfb4cc5e28e28272e768361afdbea2191",
          "message": "Enable parachain-templates zombienet tests (#9131)\n\nThis PR includes the following changes:\n- Refactor Parachain Templates workflow to run tests individually\n- Enables Zombienet Parachain Templates tests in CI\n\n---------\n\nSigned-off-by: Iulian Barbu <iulian.barbu@parity.io>\nCo-authored-by: Javier Viola <javier@parity.io>\nCo-authored-by: Javier Viola <363911+pepoviola@users.noreply.github.com>\nCo-authored-by: cmd[bot] <41898282+github-actions[bot]@users.noreply.github.com>\nCo-authored-by: Anthony Lazam <xlzm.tech@gmail.com>\nCo-authored-by: Sebastian Kunert <skunert49@gmail.com>\nCo-authored-by: Iulian Barbu <14218860+iulianbarbu@users.noreply.github.com>",
          "timestamp": "2025-07-10T06:33:27Z",
          "tree_id": "36c66069301310187811ad4f0537df4b18e2050f",
          "url": "https://github.com/paritytech/polkadot-sdk/commit/409587adfb4cc5e28e28272e768361afdbea2191"
        },
        "date": 1752133208346,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "Sent to peers",
            "value": 227.09999999999997,
            "unit": "KiB"
          },
          {
            "name": "Received from peers",
            "value": 23.800000000000004,
            "unit": "KiB"
          },
          {
            "name": "test-environment",
            "value": 0.005032093999999996,
            "unit": "seconds"
          },
          {
            "name": "dispute-coordinator",
            "value": 0.0025824143899999996,
            "unit": "seconds"
          },
          {
            "name": "dispute-distribution",
            "value": 0.008484359889999996,
            "unit": "seconds"
          }
        ]
      }
    ]
  }
}